package main

import "C"

import (
	"encoding/base64"
	"encoding/json"
	"fmt"
	"sync"
	"sync/atomic"
	"time"

	"github.com/golang/protobuf/proto"

	abci "github.com/tendermint/tendermint/abci/types"

	wasmkeeper "github.com/CosmWasm/wasmd/x/wasm/keeper"
	wasmtypes "github.com/CosmWasm/wasmd/x/wasm/types"
	"github.com/cosmos/cosmos-sdk/simapp"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/osmosis-labs/osmosis/v10/app"
	"github.com/tendermint/tendermint/crypto/secp256k1"
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

	// QueryHelper *baseapp.QueryServiceTestHelper
	// 	queryClient         types.QueryClient
	// 	msgServer           types.MsgServer
	// 	contractQueryKeeper wasmtypes.ViewKeeper
	// 	bankMsgServer       banktypes.MsgServer
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
	env.Ctx = env.App.BaseApp.NewContext(false, tmproto.Header{Height: 1, ChainID: "osmosis-1", Time: time.Now().UTC()})
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

	simapp.FundAccount(env.App.BankKeeper, env.Ctx, accAddr, coins)

	base64Priv := base64.StdEncoding.EncodeToString(priv)

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

	bz, err := proto.Marshal(codeInfo)

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

	fmt.Printf("==> %+v\n\n", msg)

	creator, err := sdk.AccAddressFromBech32(msg.Sender)
	if err != nil {
		panic(err)
	}
	admin, err := sdk.AccAddressFromBech32(msg.Admin)
	if err != nil {
		panic(err)
	}

	// TODO: use real values
	contractAddr, _, err := env.contractOpsKeeper.Instantiate(env.Ctx, msg.CodeID, creator, admin, msg.Msg, "", make(sdk.Coins, 0))
	if err != nil {
		panic(err)
	}

	return C.CString(contractAddr.String())
}

//TODO: export CwExecute
//TODO: export CwQuery
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
	header := env.Ctx.BlockHeader()

	app.BeginBlock(abci.RequestBeginBlock{Header: header})

	// DeliverTx
	resDeliverTx := env.App.DeliverTx(reqDeliverTx)

	// EndBlock
	newHeight := env.Ctx.BlockHeight() + 1
	app.EndBlock(abci.RequestEndBlock{Height: newHeight})
	env.Ctx = env.Ctx.WithBlockHeight(newHeight)

	// Commit
	app.Commit()

	bz, err := proto.Marshal(&resDeliverTx)

	if err != nil {
		panic(err)
	}

	return C.CString(string(bz))
	// see:
	// - https://github.com/osmosis-labs/osmosis/blob/ibc-rate-limit/x/ibc-rate-limit/testutil/chain.go#L56-L79)
	// - https://github.com/informalsystems/tendermint-rs/blob/9b9ed446a559c39601e83a0fb01c072642b3db06/proto/src/prost/tendermint.abci.rs#L119-L122
}

//TODO: export Query

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
func main() {}
