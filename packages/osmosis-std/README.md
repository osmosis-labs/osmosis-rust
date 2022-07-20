# osmosis-std

Rust library for interacting with osmosis. CosmWasm compatible.

## Use with CosmWasm

You can find all types generated from osmosis's protobuf in their respective module in `osmosis_std` for example.

```rs
use osmosis_std::tokenfactory::v1beta1::MsgCreateDenom;
```

With this, you can convert them to `CosmosMsg`. It is `CosmosMsg::Stargate` variant, so you need to add `features = ["stargate"]` to `cosmwasm-std`

For example:

```toml
# Cargo.toml

[dependencies]
cosmwasm-std = {version = "1.0.0", features = ["stargate"]}
```

```rs
pub fn execute(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateDenom { subdenom } => try_create_denom(env, subdenom),
    }
}

pub fn try_create_denom(env: Env, subdenom: String) -> Result<Response, ContractError> {
    let sender = env.contract.address.into();
    let msg_create_denom: CosmosMsg = MsgCreateDenom { sender, subdenom }.into();

    Ok(Response::new()
        .add_message(msg_create_denom)
        .add_attribute("method", "try_create_denom"))
}

```
