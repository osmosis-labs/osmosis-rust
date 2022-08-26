package main

import "C"

import (
	"encoding/json"
	"sync/atomic"
	"time"

	wasmtypes "github.com/CosmWasm/wasmd/x/wasm/types"
	"github.com/cosmos/cosmos-sdk/simapp"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/osmosis-labs/osmosis/v10/app"
	"github.com/tendermint/tendermint/crypto/ed25519"
	tmproto "github.com/tendermint/tendermint/proto/tendermint/types"
)

var (
	envCounter  uint64 = 0
	envRegister        = make(map[uint64]TestEnv)
)

type TestEnv struct {
	App *app.OsmosisApp
	Ctx sdk.Context
	// QueryHelper *baseapp.QueryServiceTestHelper
	// 	queryClient         types.QueryClient
	// 	msgServer           types.MsgServer
	// contractKeeper wasmtypes.ContractOpsKeeper
	// 	contractQueryKeeper wasmtypes.ViewKeeper
	// 	bankMsgServer       banktypes.MsgServer
}

func CreateRandomAccounts(numAccts int) []sdk.AccAddress {
	testAddrs := make([]sdk.AccAddress, numAccts)
	for i := 0; i < numAccts; i++ {
		pk := ed25519.GenPrivKey().PubKey()
		testAddrs[i] = sdk.AccAddress(pk.Address())
	}

	return testAddrs
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
	// env.contractKeeper = wasmkeeper.NewPermissionedKeeper(env.App.WasmKeeper, SudoAuthorizationPolicy{})

	id := atomic.LoadUint64(&envCounter)

	envRegister[id] = *env
	atomic.AddUint64(&envCounter, 1)
	return id
}

//export InitAccount
func InitAccount(envId uint64, coinsJson string) *C.char {
	env := envRegister[envId]

	var coins sdk.Coins

	if err := json.Unmarshal([]byte(coinsJson), &coins); err != nil {
		panic(err)
	}

	accAddr := sdk.AccAddress(ed25519.GenPrivKey().PubKey().Address())

	simapp.FundAccount(env.App.BankKeeper, env.Ctx, accAddr, coins)

	// String interface returns bech32 address
	return C.CString(accAddr.String())
}

//export GetAllBalances
func GetAllBalances(envId uint64, bech32Addr string) *C.char {
	env := envRegister[envId]

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

//TODO: export CwStoreCode
//TODO: export CwInstantiate
//TODO: export CwExecute
//TODO: export CwQuery
//TODO: export CwRawQuery

//TODO: export InitPool
//TODO: export Execute
//TODO: export Query

// func SetupTest(wasm string) {
// 	s := new(TestEnv)
// 	s.App = app.Setup(false)
// 	s.Ctx = s.App.BaseApp.NewContext(false, tmproto.Header{Height: 1, ChainID: "osmosis-1", Time: time.Now().UTC()})
// 	s.QueryHelper = &baseapp.QueryServiceTestHelper{
// 		GRPCQueryRouter: s.App.GRPCQueryRouter(),
// 		Ctx:             s.Ctx,
// 	}
// 	s.contractKeeper = wasmkeeper.NewPermissionedKeeper(s.App.WasmKeeper, SudoAuthorizationPolicy{})

// 	// ======================= create account

// 	s.TestAccs = CreateRandomAccounts(3)
// 	// Fund every TestAcc with 1_000_000_000_000 uosmo, uion, and uatom.
// 	fundAccsAmount := sdk.NewCoins(
// 		sdk.NewInt64Coin("uosmo", 1_000_000_000_000),
// 		sdk.NewInt64Coin("uion", 1_000_000_000_000),
// 		sdk.NewInt64Coin("uatom", 1_000_000_000_000))

// 	// ctxCheck := s.App.BaseApp.NewContext(true, tmproto.Header{})

// 	for _, acc := range s.TestAccs {
// 		s.FundAcc(acc, fundAccsAmount)
// 		s.App.BankKeeper.GetAllBalances(s.Ctx, acc)
// 	}

// 	wasmCode, err := ioutil.ReadFile(wasm)

// 	if err != nil {
// 		panic(err)
// 	}

// 	// =========== store_code
// 	codeId, _ := s.contractKeeper.Create(s.Ctx, s.TestAccs[0], wasmCode, nil)

// 	// =========== instantate
// 	instantateMsg := []byte("{}")

// 	contractAddr, _, err := s.contractKeeper.Instantiate(s.Ctx, codeId, s.TestAccs[0], s.TestAccs[0], instantateMsg, "", sdk.NewCoins())

// 	qr, err := s.App.WasmKeeper.QuerySmart(s.Ctx, contractAddr, []byte("{ \"query_token_creation_fee\": {}}"))

// 	if err != nil {
// 		panic(err)
// 	}

// 	fmt.Printf("codeId: %d\n", codeId)
// 	fmt.Printf("contract addr: %s\n", contractAddr)
// 	fmt.Printf("query result: %s\n", string(qr))
// 	// fmt.Printf("%s", s.App.BankKeeper.GetAllBalances(ctxCheck, s.TestAccs[0]).Empty())
// 	// fmt.Print(fundAccsAmount)

// }

// PLAN
// - what can run through deliver tx, let it goes through that, since
// - what could not, export them from there eg. FundAcc
// fns
// - init_test_env(..) -> Result<TestEnvId, TestEnvError?>
// - clone_test_env(id: TestEnvId) -> Result<TestEnvId, TestEnvError>
// - pub fn_execute_helper! { mint_coins: ExecuteMsg::XXX } // pub fn mint_coins(ExecuteMsg::XXX) -> Result<Resposne, TestEnvError
// - pub fn_query_helper! { (query_wat: QueryMsg::Wat -> WatResponse) }
// - set_block_info
// pub struct BlockInfo {
// 	pub height: u64,
// 	pub time: Timestamp,
// 	pub chain_id: String,
// }
// notes:
// - consistent errors interface
// - speaks primite / json to each other

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
