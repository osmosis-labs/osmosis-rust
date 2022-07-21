# osmosis-stargate

start [localosmis](https://docs.osmosis.zone/developing/tools/localosmosis.html) then deploy and generate client:

```sh
beaker wasm deploy tokenfactory --signer-account test1 --no-wasm-opt --admin signer --raw '{}'
beaker wasm ts-gen tokenfactory
```

try it out on beaker console

```js
sc = tokenfactory.signer(test1);
await sc.createDenom({ subdenom: "token1" }, "auto", undefined, [
  { denom: "uosmo", amount: "10000000" },
]);
```

since stargate query whitelisting is still in progress, you can check the result via `osmosisd`.

```js
// get address from contract info for later query
(await tokenfactory.getInfo()).address; // => <contract_addr>
```

```sh
osmosisd query tokenfactory denoms-from-creator <contract_addr>

# denoms:
# - factory/<contract_addr>/token1
```
