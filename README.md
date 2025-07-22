# neutron-std

Rust libraries for Neutron. The following table shows every published crates maintained in this repository:

| Crate                                             | Description                                                                                                                   | Crates.io                                                                                                                                 | Docs                                                                                        |
|---------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------|
| [neutron-std](packages/neutron-std)               | Neutron's proto-generated types and helpers for interacting with the appchain. Compatible with CosmWasm contract.             | [![neutron-std on crates.io](https://img.shields.io/crates/v/neutron-std.svg)](https://crates.io/crates/neutron-std)                      | [![Docs](https://docs.rs/neutron-std/badge.svg)](https://docs.rs/neutron-std)               |
| [neutron-std-derive](packages/neutron-std-derive) | Procedural macro for augmenting proto-generated types to create better developer ergonomics. Internally used by `neutron-std` | [![neutron-std-derive on crates.io](https://img.shields.io/crates/v/neutron-std-derive.svg)](https://crates.io/crates/neutron-std-derive) | [![Docs](https://docs.rs/neutron-std-derive/badge.svg)](https://docs.rs/neutron-std-derive) |

---

This repo also contains [`proto-build`](./proto-build) package which is used for autogenrating rust types from proto.
