# osmosis-std

Rust library for interacting with osmosis. CosmWasm compatible.

## CosmWasm stargate message and stargate query

You can find all types and querier generated from osmosis's protobuf in their respective module in `osmosis_std`.

[Full working example contract can be found here.](/examples/cosmwasm/contracts/osmosis-stargate)

### Publishing Osmosis' message from CosmWasm Contract

```rs
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgCreateDenom;

// ..

pub fn try_create_denom(env: Env, subdenom: String) -> Result<Response, ContractError> {
    let sender = env.contract.address.into();

    // construct message and convet them into cosmos message
    // (notice `CosmosMsg` type and `.into()`)
    let msg_create_denom: CosmosMsg = MsgCreateDenom { sender, subdenom }.into();

    Ok(Response::new()
        .add_message(msg_create_denom)
        .add_attribute("method", "try_create_denom"))
}

```

## Querying Osmosis' module

```rs
use osmosis_std::types::osmosis::tokenfactory::v1beta1::TokenfactoryQuerier;

// .. 

fn query_creator_denoms(deps: Deps, env: Env) -> StdResult<QueryCreatorDenomsResponse> {
    // create `TokenfactoryQuerier`
    let tokenfactory = TokenfactoryQuerier::new(deps.querier);

    // `TokenfactoryQuerier` has all the fns for querying the module
    let res = tokenfactory.denoms_from_creator(env.contract.address.into())?;

    Ok(QueryCreatorDenomsResponse { denoms: res.denoms })
}
```



## Non-CosmWasm Client
(WIP)