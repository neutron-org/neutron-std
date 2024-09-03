# neutron-std

[![neutron-std on crates.io](https://img.shields.io/crates/v/neutron-std.svg)](https://crates.io/crates/neutron-std) [![Docs](https://docs.rs/neutron-std/badge.svg)](https://docs.rs/neutron-std)

Neutron's proto-generated types and helpers for interacting with the appchain. Compatible with CosmWasm contract.

## CosmWasm stargate message and stargate query

You can find all types and querier generated from Neutron's protobuf in their respective module in `neutron_std`.

### Executing Neutron messages from CosmWasm Contract

```rust
use cosmwasm_std::{CosmosMsg, Response, Env};
use neutron_std::types::osmosis::tokenfactory::v1beta1::MsgCreateDenom;

# type ContractError = cosmwasm_std::StdError;
// ..

pub fn try_create_denom(env: Env, subdenom: String) -> Result<Response, ContractError> {
    let sender = env.contract.address.into();

    // construct message and convert them into cosmos message
    // (notice `CosmosMsg` type and `.into()`)
    let msg_create_denom: CosmosMsg = MsgCreateDenom { sender, subdenom }.into();

    Ok(Response::new()
        .add_message(msg_create_denom)
        .add_attribute("method", "try_create_denom"))
}

```
