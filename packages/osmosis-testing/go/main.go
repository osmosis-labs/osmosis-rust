package main

import "C"

import (
	// std
	"encoding/base64"
	"encoding/json"
	"fmt"
	"strings"
	"sync"
	"time"

	// helpers
	"github.com/golang/protobuf/proto"
	"github.com/pkg/errors"

	// tendermint
	abci "github.com/tendermint/tendermint/abci/types"
	"github.com/tendermint/tendermint/libs/log"
	tmproto "github.com/tendermint/tendermint/proto/tendermint/types"
	tmtypes "github.com/tendermint/tendermint/proto/tendermint/types"
	dbm "github.com/tendermint/tm-db"

	// cosmos sdk
	"github.com/cosmos/cosmos-sdk/baseapp"
	"github.com/cosmos/cosmos-sdk/crypto/keys/secp256k1"
	"github.com/cosmos/cosmos-sdk/simapp"
	sdk "github.com/cosmos/cosmos-sdk/types"
	slashingtypes "github.com/cosmos/cosmos-sdk/x/slashing/types"
	"github.com/cosmos/cosmos-sdk/x/staking"
	stakingtypes "github.com/cosmos/cosmos-sdk/x/staking/types"

	// wasmd
	wasmkeeper "github.com/CosmWasm/wasmd/x/wasm/keeper"
	wasmtypes "github.com/CosmWasm/wasmd/x/wasm/types"

	// osmosis
	"github.com/osmosis-labs/osmosis/v11/app"
	"github.com/osmosis-labs/osmosis/v11/x/gamm/pool-models/balancer"

	// cosmwasm-testing
	"github.com/osmosis-labs/osmosis/v11/cosmwasm-testing/result"
)

var (
	envCounter  uint64 = 0
	envRegister        = sync.Map{}
	mu          sync.Mutex
)

type TestEnv struct {
	App *app.OsmosisApp
	Ctx sdk.Context

	// cosmwasm related keepers
	contractOpsKeeper wasmtypes.ContractOpsKeeper
	QueryHelper       *baseapp.QueryServiceTestHelper
}

func SetupOsmosisApp() *app.OsmosisApp {
	db := dbm.NewMemDB()
	appInstance := app.NewOsmosisApp(log.NewNopLogger(), db, nil, true, map[int64]bool{}, app.DefaultNodeHome, 5, app.MakeEncodingConfig(), simapp.EmptyAppOptions{}, app.GetWasmEnabledProposals(), app.EmptyWasmOpts)
	genesisState := app.NewDefaultGenesisState()
	stateBytes, err := json.MarshalIndent(genesisState, "", " ")
	if err != nil {
		panic(err)
	}

	// replace sdk.DefaultDenom with "uosmo", a bit of a hack, needs improvement
	stateBytes = []byte(strings.Replace(string(stateBytes), "\"stake\"", "\"uosmo\"", -1))

	appInstance.InitChain(
		abci.RequestInitChain{
			Validators:      []abci.ValidatorUpdate{},
			ConsensusParams: simapp.DefaultConsensusParams,
			AppStateBytes:   stateBytes,
		},
	)

	return appInstance
}

//export InitTestEnv
func InitTestEnv() uint64 {
	// Allow testing unoptimized contract
	wasmtypes.MaxWasmSize = 1024 * 1024 * 1024 * 1024 * 1024
	// Temp fix for concurrency issue
	mu.Lock()
	defer mu.Unlock()

	env := new(TestEnv)
	env.App = SetupOsmosisApp()

	env.Ctx = env.App.BaseApp.NewContext(false, tmproto.Header{Height: 0, ChainID: "osmosis-1", Time: time.Now().UTC()})

	env.BeginNewBlock(false)

	reqEndBlock := abci.RequestEndBlock{Height: env.Ctx.BlockHeight()}
	env.App.EndBlock(reqEndBlock)
	env.App.Commit()

	env.QueryHelper = &baseapp.QueryServiceTestHelper{
		GRPCQueryRouter: env.App.GRPCQueryRouter(),
		Ctx:             env.Ctx,
	}

	env.contractOpsKeeper = wasmkeeper.NewPermissionedKeeper(env.App.WasmKeeper, SudoAuthorizationPolicy{})

	envCounter += 1
	id := envCounter

	envRegister.Store(id, *env)

	return id
}

//export InitAccount
func InitAccount(envId uint64, coinsJson string) *C.char {
	env := loadEnv(envId)

	var coins sdk.Coins

	if err := json.Unmarshal([]byte(coinsJson), &coins); err != nil {
		panic(err)
	}

	priv := secp256k1.GenPrivKey()
	accAddr := sdk.AccAddress(priv.PubKey().Address())

	env.BeginNewBlock(false)
	err := simapp.FundAccount(env.App.BankKeeper, env.Ctx, accAddr, coins)
	if err != nil {
		panic(errors.Wrapf(err, "Failed to fund account"))
	}
	reqEndBlock := abci.RequestEndBlock{Height: env.Ctx.BlockHeight()}
	env.App.EndBlock(reqEndBlock)

	env.App.Commit()

	base64Priv := base64.StdEncoding.EncodeToString(priv.Bytes())

	envRegister.Store(envId, env)

	return C.CString(base64Priv)
}

//export GetAllBalances
func GetAllBalances(envId uint64, bech32Addr string) *C.char {
	env := loadEnv(envId)

	addr, err := sdk.AccAddressFromBech32(bech32Addr)

	if err != nil {
		panic(err)
	}

	bal := env.App.BankKeeper.GetAllBalances(env.Ctx, addr)
	bz, err := json.Marshal(bal)

	if err != nil {
		panic(err)
	}

	return C.CString(string(bz))
}

//export CwStoreCode
func CwStoreCode(envId uint64, bech32Addr string, base64Wasm string) uint64 {
	env := loadEnv(envId)

	addr, err := sdk.AccAddressFromBech32(bech32Addr)
	if err != nil {
		panic(err)
	}

	wasm, err := base64.StdEncoding.DecodeString(base64Wasm)
	if err != nil {
		panic(err)
	}

	// TODO: expose access config
	codeId, err := env.contractOpsKeeper.Create(env.Ctx, addr, wasm, nil)
	if err != nil {
		panic(err)
	}

	return codeId
}

//export CwGetCodeInfo
func CwGetCodeInfo(envId uint64, codeId uint64) *C.char {
	env := loadEnv(envId)

	codeInfo := env.App.WasmKeeper.GetCodeInfo(env.Ctx, codeId)

	if codeInfo == nil {
		return nil
	}

	bz, err := codeInfo.Marshal()

	if err != nil {
		panic(err)
	}

	return C.CString(string(bz))
}

//export CwInstantiate
func CwInstantiate(envId uint64, base64msgInstantiateContract string) *C.char {
	env := loadEnv(envId)

	msgInstantiateContractBytes, err := base64.StdEncoding.DecodeString(base64msgInstantiateContract)
	if err != nil {
		panic(err)
	}

	msg := wasmtypes.MsgInstantiateContract{}
	err = proto.Unmarshal(msgInstantiateContractBytes, &msg)
	if err != nil {
		panic(err)
	}

	creator, err := sdk.AccAddressFromBech32(msg.Sender)
	if err != nil {
		panic(err)
	}

	admin, err := sdk.AccAddressFromBech32(msg.Admin)
	if err != nil {
		panic(err)
	}

	contractAddr, _, err := env.contractOpsKeeper.Instantiate(env.Ctx, msg.CodeID, creator, admin, msg.Msg, msg.Label, msg.Funds)
	if err != nil {
		panic(errors.Wrapf(err, "Failed to instantiate contract"))
	}

	return C.CString(contractAddr.String())
}

//export CwGetContractInfo
func CwGetContractInfo(envId uint64, bech32ContractAddr string) *C.char {
	env := loadEnv(envId)

	contractAddr, err := sdk.AccAddressFromBech32(bech32ContractAddr)
	if err != nil {
		panic(err)
	}

	contractInfo := env.App.WasmKeeper.GetContractInfo(env.Ctx, contractAddr)

	if contractInfo == nil {
		return nil
	}

	bz, err := proto.Marshal(contractInfo)

	if err != nil {
		panic(err)
	}

	return C.CString(string(bz))
}

//export CwExecute
func CwExecute(envId uint64, base64MsgExecuteContract string) *C.char {
	env := loadEnv(envId)

	msgExecuteContractBytes, err := base64.StdEncoding.DecodeString(base64MsgExecuteContract)
	if err != nil {
		return encodeErrToResultBytes(err)
	}

	msg := wasmtypes.MsgExecuteContract{}
	err = proto.Unmarshal(msgExecuteContractBytes, &msg)
	if err != nil {
		return encodeErrToResultBytes(err)
	}

	contractAddress, err := sdk.AccAddressFromBech32(msg.Contract)
	if err != nil {
		return encodeErrToResultBytes(err)
	}

	senderAddress, err := sdk.AccAddressFromBech32(msg.Sender)
	if err != nil {
		return encodeErrToResultBytes(err)
	}

	// env.BeginNewBlock(false)
	res, err := env.contractOpsKeeper.Execute(env.Ctx, contractAddress, senderAddress, msg.Msg, msg.Funds)
	// reqEndBlock := abci.RequestEndBlock{Height: env.Ctx.BlockHeight()}
	// env.App.EndBlock(reqEndBlock)

	// env.App.Commit()

	if err != nil {
		return encodeErrToResultBytes(err)
	}

	return encodeBytesResultBytes(res)
}

//export CwQuery
func CwQuery(envId uint64, bech32ContractAddress, queryMsgJson string) *C.char {
	env := loadEnv(envId)

	queryMsgBytes := []byte(queryMsgJson)
	contractAddress, err := sdk.AccAddressFromBech32(bech32ContractAddress)
	if err != nil {
		return encodeErrToResultBytes(err)
	}

	res, err := env.App.WasmKeeper.QuerySmart(env.Ctx, contractAddress, queryMsgBytes)

	if err != nil {
		return encodeErrToResultBytes(err)
	}

	return encodeBytesResultBytes(res)
}

//TODO: export CwRawQuery

//export CommitTx
func CommitTx(envId uint64, base64ReqDeliverTx string) *C.char {
	env := loadEnv(envId)
	app := env.App

	reqDeliverTxBytes, err := base64.StdEncoding.DecodeString(base64ReqDeliverTx)
	if err != nil {
		panic(err)
	}

	reqDeliverTx := abci.RequestDeliverTx{}
	err = proto.Unmarshal(reqDeliverTxBytes, &reqDeliverTx)
	if err != nil {
		panic(err)
	}

	// TODO: try using app.GetContextForDeliverTx
	// ctx := app.GetContextForDeliverTx(reqDeliverTx.Tx)

	// BeginBlock
	env.BeginNewBlock(false)

	// DeliverTx
	resDeliverTx := env.App.DeliverTx(reqDeliverTx)

	// EndBlock
	reqEndBlock := abci.RequestEndBlock{Height: env.Ctx.BlockHeight()}
	env.App.EndBlock(reqEndBlock)

	// Commit
	app.Commit()

	bz, err := proto.Marshal(&resDeliverTx)

	if err != nil {
		panic(err)
	}

	envRegister.Store(envId, env)

	return C.CString(string(bz))
}

//export GAMMCreateBalancerPool
func GAMMCreateBalancerPool(envId uint64, base64MsgCreateBalancerPool string) uint64 {
	env := loadEnv(envId)

	msgCreateBalancerPoolBytes, err := base64.StdEncoding.DecodeString(base64MsgCreateBalancerPool)
	if err != nil {
		panic(err)
	}

	msg := balancer.MsgCreateBalancerPool{}
	err = proto.Unmarshal(msgCreateBalancerPoolBytes, &msg)
	if err != nil {
		panic(err)
	}

	poolId, err := env.App.GAMMKeeper.CreatePool(env.Ctx, msg)
	if err != nil {
		panic(err)
	}

	return poolId
}

//export GAMMGetTotalPoolLiquidity
func GAMMGetTotalPoolLiquidity(envId uint64, poolId uint64) *C.char {
	env := loadEnv(envId)
	pool, err := env.App.GAMMKeeper.GetPoolAndPoke(env.Ctx, poolId)
	if err != nil {
		panic(err)
	}

	liq := pool.GetTotalPoolLiquidity(env.Ctx)
	bz, err := liq.MarshalJSON()

	if err != nil {
		panic(errors.Wrapf(err, "Failed to marshal total pool liquidity:\n %s", liq))
	}

	return C.CString(string(bz))
}

// ========= utils =========
func (s *TestEnv) BeginNewBlock(executeNextEpoch bool) {
	var valAddr []byte

	validators := s.App.StakingKeeper.GetAllValidators(s.Ctx)
	if len(validators) >= 1 {
		valAddrFancy, err := validators[0].GetConsAddr()
		if err != nil {
			panic(err)
		}
		valAddr = valAddrFancy.Bytes()
	} else {
		valAddrFancy := s.SetupValidator(stakingtypes.Bonded)
		validator, _ := s.App.StakingKeeper.GetValidator(s.Ctx, valAddrFancy)
		valAddr2, _ := validator.GetConsAddr()
		valAddr = valAddr2.Bytes()
	}

	s.BeginNewBlockWithProposer(executeNextEpoch, valAddr)
}

// BeginNewBlockWithProposer begins a new block with a proposer.
func (s *TestEnv) BeginNewBlockWithProposer(executeNextEpoch bool, proposer sdk.ValAddress) {
	validator, found := s.App.StakingKeeper.GetValidator(s.Ctx, proposer)

	if !found {
		panic("validator not found")
	}

	valConsAddr, err := validator.GetConsAddr()
	if err != nil {
		panic(err)
	}

	valAddr := valConsAddr.Bytes()

	epochIdentifier := s.App.SuperfluidKeeper.GetEpochIdentifier(s.Ctx)
	epoch := s.App.EpochsKeeper.GetEpochInfo(s.Ctx, epochIdentifier)
	newBlockTime := s.Ctx.BlockTime().Add(5 * time.Second)
	if executeNextEpoch {
		newBlockTime = s.Ctx.BlockTime().Add(epoch.Duration).Add(time.Second)
	}

	header := tmtypes.Header{ChainID: "osmosis-1", Height: s.Ctx.BlockHeight() + 1, Time: newBlockTime}
	newCtx := s.Ctx.WithBlockTime(newBlockTime).WithBlockHeight(s.Ctx.BlockHeight() + 1)
	s.Ctx = newCtx
	lastCommitInfo := abci.LastCommitInfo{
		Votes: []abci.VoteInfo{{
			Validator:       abci.Validator{Address: valAddr, Power: 1000},
			SignedLastBlock: true,
		}},
	}
	reqBeginBlock := abci.RequestBeginBlock{Header: header, LastCommitInfo: lastCommitInfo}

	// fmt.Println("beginning block ", s.Ctx.BlockHeight())
	s.App.BeginBlock(reqBeginBlock)
	s.Ctx = s.App.NewContext(false, reqBeginBlock.Header)
}

func (s *TestEnv) SetupValidator(bondStatus stakingtypes.BondStatus) sdk.ValAddress {
	valPub := secp256k1.GenPrivKey().PubKey()
	valAddr := sdk.ValAddress(valPub.Address())
	bondDenom := s.App.StakingKeeper.GetParams(s.Ctx).BondDenom
	selfBond := sdk.NewCoins(sdk.Coin{Amount: sdk.NewInt(100), Denom: bondDenom})

	err := simapp.FundAccount(s.App.BankKeeper, s.Ctx, sdk.AccAddress(valPub.Address()), selfBond)
	if err != nil {
		panic(errors.Wrapf(err, "Failed to fund account"))
	}

	stakingHandler := staking.NewHandler(*s.App.StakingKeeper)
	stakingCoin := sdk.NewCoin(bondDenom, selfBond[0].Amount)
	ZeroCommission := stakingtypes.NewCommissionRates(sdk.ZeroDec(), sdk.ZeroDec(), sdk.ZeroDec())
	msg, err := stakingtypes.NewMsgCreateValidator(valAddr, valPub, stakingCoin, stakingtypes.Description{}, ZeroCommission, sdk.OneInt())
	// s.Require().NoError(err)
	_, err = stakingHandler(s.Ctx, msg)
	// s.Require().NoError(err)
	// s.Require().NotNil(res)

	val, _ := s.App.StakingKeeper.GetValidator(s.Ctx, valAddr)
	// s.Require().True(found)

	val = val.UpdateStatus(bondStatus)
	s.App.StakingKeeper.SetValidator(s.Ctx, val)

	consAddr, err := val.GetConsAddr()
	// s.Suite.Require().NoError(err)

	signingInfo := slashingtypes.NewValidatorSigningInfo(
		consAddr,
		s.Ctx.BlockHeight(),
		0,
		time.Unix(0, 0),
		false,
		0,
	)
	s.App.SlashingKeeper.SetValidatorSigningInfo(s.Ctx, consAddr, signingInfo)

	return valAddr
}

func loadEnv(envId uint64) TestEnv {
	item, ok := envRegister.Load(envId)
	env := TestEnv(item.(TestEnv))
	if !ok {
		panic(fmt.Sprintf("env not found: %d", envId))
	}
	return env
}

type SudoAuthorizationPolicy struct{}

func (p SudoAuthorizationPolicy) CanCreateCode(config wasmtypes.AccessConfig, actor sdk.AccAddress) bool {
	return true
}

func (p SudoAuthorizationPolicy) CanInstantiateContract(config wasmtypes.AccessConfig, actor sdk.AccAddress) bool {
	return true
}

func (p SudoAuthorizationPolicy) CanModifyContract(admin, actor sdk.AccAddress) bool {
	return true
}

func encodeErrToResultBytes(err error) *C.char {
	return C.CString(result.EncodeResultFromError(err))
}

func encodeBytesResultBytes(bytes []byte) *C.char {
	return C.CString(result.EncodeResultFromOk(bytes))
}

// must define main for ffi build
func main() {}
