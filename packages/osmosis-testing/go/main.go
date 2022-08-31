package main

import "C"

import (
	"encoding/base64"
	"encoding/json"
	"fmt"
	"sync"
	"sync/atomic"
	"time"

	"github.com/cosmos/cosmos-sdk/baseapp"
	"github.com/cosmos/cosmos-sdk/crypto/keys/secp256k1"
	"github.com/cosmos/cosmos-sdk/simapp"
	sdk "github.com/cosmos/cosmos-sdk/types"
	slashingtypes "github.com/cosmos/cosmos-sdk/x/slashing/types"
	"github.com/cosmos/cosmos-sdk/x/staking"
	stakingtypes "github.com/cosmos/cosmos-sdk/x/staking/types"
	abci "github.com/tendermint/tendermint/abci/types"
	tmtypes "github.com/tendermint/tendermint/proto/tendermint/types"

	"github.com/osmosis-labs/osmosis/v11/app"
	"github.com/osmosis-labs/osmosis/v11/x/gamm/pool-models/balancer"

	"github.com/pkg/errors"

	"github.com/golang/protobuf/proto"

	wasmkeeper "github.com/CosmWasm/wasmd/x/wasm/keeper"
	wasmtypes "github.com/CosmWasm/wasmd/x/wasm/types"

	tmproto "github.com/tendermint/tendermint/proto/tendermint/types"
)

var (
	envCounter  uint64 = 0
	envRegister        = sync.Map{}
)

type TestEnv struct {
	App *app.OsmosisApp
	Ctx sdk.Context

	// cosmwasm related keepers
	contractOpsKeeper wasmtypes.ContractOpsKeeper

	QueryHelper *baseapp.QueryServiceTestHelper
	// 	queryClient         types.QueryClient
	// 	msgServer           types.MsgServer
	// bankMsgServer banktypes.MsgServer
}

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
	stakingCoin := sdk.NewCoin(sdk.DefaultBondDenom, selfBond[0].Amount)
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

//export InitTestEnv
func InitTestEnv() uint64 {
	env := new(TestEnv)
	env.App = app.Setup(false)
	env.Ctx = env.App.BaseApp.NewContext(false, tmproto.Header{Height: 0, ChainID: "osmosis-1", Time: time.Now().UTC()})

	env.BeginNewBlock(false)

	reqEndBlock := abci.RequestEndBlock{Height: env.Ctx.BlockHeight()}
	env.App.EndBlock(reqEndBlock)
	env.App.Commit()

	// env.QueryHelper = &baseapp.QueryServiceTestHelper{
	// 	GRPCQueryRouter: env.App.GRPCQueryRouter(),
	// 	Ctx:             env.Ctx,
	// }

	// TODO: set limit to 1G
	env.contractOpsKeeper = wasmkeeper.NewPermissionedKeeper(env.App.WasmKeeper, SudoAuthorizationPolicy{})

	id := atomic.LoadUint64(&envCounter)
	envRegister.Store(id, *env)
	atomic.AddUint64(&envCounter, 1)

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
		panic(err)
	}

	msg := wasmtypes.MsgExecuteContract{}
	err = proto.Unmarshal(msgExecuteContractBytes, &msg)
	if err != nil {
		panic(err)
	}

	contractAddress, err := sdk.AccAddressFromBech32(msg.Contract)
	if err != nil {
		panic(err)
	}

	senderAddress, err := sdk.AccAddressFromBech32(msg.Sender)
	if err != nil {
		panic(err)
	}

	res, err := env.contractOpsKeeper.Execute(env.Ctx, contractAddress, senderAddress, msg.Msg, msg.Funds)

	if err != nil {
		panic(err)
	}

	return C.CString(base64.StdEncoding.EncodeToString(res))
}

//export CwQuery
func CwQuery(envId uint64, bech32ContractAddress, queryMsgJson string) *C.char {
	env := loadEnv(envId)

	queryMsgBytes := []byte(queryMsgJson)
	contractAddress, err := sdk.AccAddressFromBech32(bech32ContractAddress)
	if err != nil {
		panic(err)
	}

	res, err := env.App.WasmKeeper.QuerySmart(env.Ctx, contractAddress, queryMsgBytes)

	if err != nil {
		panic(err)
	}

	return C.CString(string(res))
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

//TODO: export Query

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

// must define main for ffi build
func main() {
	// envId := InitTestEnv()
	// InitAccount(envId, "[{ \"denom\": \"uatom\", \"amount\": \"10000000000000\"}, { \"denom\": \"uosmo\", \"amount\": \"10000000000000\"}]")
	// InitAccount(envId, "[{ \"denom\": \"uatom\", \"amount\": \"10000000000000\"}, { \"denom\": \"uosmo\", \"amount\": \"10000000000000\"}]")
	// // - priv -> bytes -> address
	// privBytes, _ := base64.StdEncoding.DecodeString(C.GoString(priv))

	// addr := secp256k1.PrivKey(privBytes).PubKey().Address()

	// initialLiquidity := sdk.NewCoins(sdk.NewInt64Coin("uosmo", 1_000), sdk.NewInt64Coin("uatom", 1_000))

	// var poolAssets []balancer.PoolAsset

	// for _, asset := range initialLiquidity {
	// 	poolAssets = append(poolAssets, balancer.PoolAsset{
	// 		Weight: sdk.NewInt(1),
	// 		Token:  asset,
	// 	})
	// }

	// poolParams := balancer.PoolParams{
	// 	SwapFee: sdk.NewDecWithPrec(1, 2),
	// 	ExitFee: sdk.NewDecWithPrec(1, 2),
	// }

	// acc, _ := sdk.AccAddressFromHex(addr.String())
	// msg := balancer.NewMsgCreateBalancerPool(acc, poolParams, poolAssets, "")
	// m, _ := msg.Marshal()

	// mb64 := base64.StdEncoding.EncodeToString(m)

	// GAMMCreateBalancerPool(envId, mb64)

	// fmt.Printf("%v", C.GoString(priv))
}
