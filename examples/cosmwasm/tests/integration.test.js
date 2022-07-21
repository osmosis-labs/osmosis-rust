const fs = require("fs");
const path = require("path");
const { SigningCosmWasmClient, Secp256k1HdWallet } = require("cosmwasm");
const { stringToPath } = require("@cosmjs/crypto");
const { OsmosisStargateContract } = require("osmosis-stargate-sdk");
const { OsmosisStargateClient } = OsmosisStargateContract;

jest.setTimeout(100000);

let contractAddr;
let client;
let osmosisStargate;

beforeAll(async () => {
  contractAddr = JSON.parse(
    fs.readFileSync(path.join(__dirname, "..", ".beaker", "state.local.json"))
  ).local["osmosis-stargate"].addresses.default;

  const walletConf = {
    prefix: "osmo",
    hdPaths: [stringToPath("m/44'/118'/0'/0/0")],
  };
  const mnemonic =
    "notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius";
  const signer = await Secp256k1HdWallet.fromMnemonic(mnemonic, walletConf);

  const sender = (await signer.getAccounts())[0].address;

  const rpcEndpoint = "http://localhost:26657";

  client = await SigningCosmWasmClient.connectWithSigner(rpcEndpoint, signer, {
    gasPrice: "1uosmo",
  });

  osmosisStargate = new OsmosisStargateClient(client, sender, contractAddr);
});

test("tokenfactory createDenom", async () => {
  const id = Math.random();
  const subdenom = `token-${id}`.replace("0.", "");
  const res = await osmosisStargate.createDenom(
    { subdenom },
    "auto",
    undefined,
    [{ denom: "uosmo", amount: "10000000" }]
  );

  expect(getEventAttr(res, "create_denom", "new_token_denom")).toBe(
    `factory/${contractAddr}/${subdenom}`
  );
});

const getEventAttr = (res, type, key) =>
  res.logs[0].events
    .find((e) => e.type === type)
    .attributes.find((a) => a.key === key).value;
