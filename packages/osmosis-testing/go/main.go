package main

import "C"

import (
	// std
	"encoding/base64"
	"encoding/json"
	"fmt"
	"sync"
	"time"

	// helpers
	"github.com/golang/protobuf/proto"
	"github.com/pkg/errors"

	// tendermint
	abci "github.com/tendermint/tendermint/abci/types"
	tmproto "github.com/tendermint/tendermint/proto/tendermint/types"

	// cosmos sdk
	"github.com/cosmos/cosmos-sdk/baseapp"
	"github.com/cosmos/cosmos-sdk/crypto/keys/secp256k1"
	"github.com/cosmos/cosmos-sdk/simapp"
	sdk "github.com/cosmos/cosmos-sdk/types"

	// wasmd
	wasmkeeper "github.com/CosmWasm/wasmd/x/wasm/keeper"
	wasmtypes "github.com/CosmWasm/wasmd/x/wasm/types"

	// osmosis

	"github.com/osmosis-labs/osmosis/v11/x/gamm/pool-models/balancer"

	// cosmwasm-testing
	"github.com/osmosis-labs/osmosis/v11/cosmwasm-testing/result"
	"github.com/osmosis-labs/osmosis/v11/cosmwasm-testing/testenv"
)

var (
	envCounter  uint64 = 0
	envRegister        = sync.Map{}
	mu          sync.Mutex
)

//export InitTestEnv
func InitTestEnv() uint64 {
	// Allow testing unoptimized contract
	wasmtypes.MaxWasmSize = 1024 * 1024 * 1024 * 1024 * 1024
	// Temp fix for concurrency issue
	mu.Lock()
	defer mu.Unlock()

	env := new(testenv.TestEnv)
	env.App = testenv.SetupOsmosisApp()

	env.Ctx = env.App.BaseApp.NewContext(false, tmproto.Header{Height: 0, ChainID: "osmosis-1", Time: time.Now().UTC()})

	env.BeginNewBlock(false)

	reqEndBlock := abci.RequestEndBlock{Height: env.Ctx.BlockHeight()}
	env.App.EndBlock(reqEndBlock)
	env.App.Commit()

	env.QueryHelper = &baseapp.QueryServiceTestHelper{
		GRPCQueryRouter: env.App.GRPCQueryRouter(),
		Ctx:             env.Ctx,
	}

	env.ContractOpsKeeper = wasmkeeper.NewPermissionedKeeper(env.App.WasmKeeper, SudoAuthorizationPolicy{})

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

//export BeginBlock
func BeginBlock(envId uint64) {
	env := loadEnv(envId)
	env.BeginNewBlock(false)
	envRegister.Store(envId, env)
}

//export EndBlock
func EndBlock(envId uint64) {
	env := loadEnv(envId)
	reqEndBlock := abci.RequestEndBlock{Height: env.Ctx.BlockHeight()}
	env.App.EndBlock(reqEndBlock)
	env.App.Commit()
	envRegister.Store(envId, env)
}

//export Execute
func Execute(envId uint64, base64ReqDeliverTx string) *C.char {
	env := loadEnv(envId)
	reqDeliverTxBytes, err := base64.StdEncoding.DecodeString(base64ReqDeliverTx)
	if err != nil {
		panic(err)
	}

	reqDeliverTx := abci.RequestDeliverTx{}
	err = proto.Unmarshal(reqDeliverTxBytes, &reqDeliverTx)
	if err != nil {
		panic(err)
	}

	resDeliverTx := env.App.DeliverTx(reqDeliverTx)
	bz, err := proto.Marshal(&resDeliverTx)

	if err != nil {
		panic(err)
	}

	envRegister.Store(envId, env)

	return C.CString(string(bz))
}

//// export Query
// func Query(envId uint64, path, base64ReqDeliverTx string) *C.char {
// 	env := loadEnv(envId)
// 	reqDeliverTxBytes, err := base64.StdEncoding.DecodeString(base64ReqDeliverTx)
// 	if err != nil {
// 		panic(err)
// 	}

// 	reqDeliverTx := abci.RequestDeliverTx{}
// 	err = proto.Unmarshal(reqDeliverTxBytes, &reqDeliverTx)
// 	if err != nil {
// 		panic(err)
// 	}

// 	// env.App.GRPCQueryRouter().Route(path)()
// 	resDeliverTx := env.App.DeliverTx(reqDeliverTx)
// 	bz, err := proto.Marshal(&resDeliverTx)

// 	if err != nil {
// 		panic(err)
// 	}

// 	envRegister.Store(envId, env)

// 	return C.CString(string(bz))
// }

//export AccountSequence
func AccountSequence(envId uint64, bech32Address string) uint64 {
	env := loadEnv(envId)

	addr, err := sdk.AccAddressFromBech32(bech32Address)

	if err != nil {
		panic(errors.Wrapf(err, "Must always be valid bech32"))
	}

	seq, err := env.App.AppKeepers.AccountKeeper.GetSequence(env.Ctx, addr)

	if err != nil {
		panic(errors.Wrapf(err, "Account must always be initialized"))
	}

	return seq
}

//export AccountNumber
func AccountNumber(envId uint64, bech32Address string) uint64 {
	env := loadEnv(envId)

	addr, err := sdk.AccAddressFromBech32(bech32Address)

	if err != nil {
		panic(errors.Wrapf(err, "Must always be valid bech32"))
	}

	acc := env.App.AppKeepers.AccountKeeper.GetAccount(env.Ctx, addr)
	return acc.GetAccountNumber()
}

//export Simulate
func Simulate(envId uint64, base64TxBytes string) *C.char { // => base64GasInfo
	env := loadEnv(envId)

	txBytes, err := base64.StdEncoding.DecodeString(base64TxBytes)
	if err != nil {
		panic(err)
	}

	gasInfo, _, err := env.App.Simulate(txBytes)

	if err != nil {
		panic(errors.Wrapf(err, "Simulation failed"))
	}

	bz, err := proto.Marshal(&gasInfo)
	if err != nil {
		panic(err)
	}

	return C.CString(string(bz))
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
	env.BeginNewBlock(false)

	codeId, err := env.ContractOpsKeeper.Create(env.Ctx, addr, wasm, nil)
	if err != nil {
		panic(err)
	}
	reqEndBlock := abci.RequestEndBlock{Height: env.Ctx.BlockHeight()}
	env.App.EndBlock(reqEndBlock)
	env.App.Commit()
	envRegister.Store(envId, env)

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

	env.BeginNewBlock(false)

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

	contractAddr, _, err := env.ContractOpsKeeper.Instantiate(env.Ctx, msg.CodeID, creator, admin, msg.Msg, msg.Label, msg.Funds)
	if err != nil {
		panic(errors.Wrapf(err, "Failed to instantiate contract"))
	}
	reqEndBlock := abci.RequestEndBlock{Height: env.Ctx.BlockHeight()}
	env.App.EndBlock(reqEndBlock)
	env.App.Commit()
	envRegister.Store(envId, env)

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

	env.BeginNewBlock(false)
	res, err := env.ContractOpsKeeper.Execute(env.Ctx, contractAddress, senderAddress, msg.Msg, msg.Funds)
	reqEndBlock := abci.RequestEndBlock{Height: env.Ctx.BlockHeight()}
	env.App.EndBlock(reqEndBlock)

	env.App.Commit()
	envRegister.Store(envId, env)

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

func loadEnv(envId uint64) testenv.TestEnv {
	item, ok := envRegister.Load(envId)
	env := testenv.TestEnv(item.(testenv.TestEnv))
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
