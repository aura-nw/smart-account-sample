# smart-account-sample

a smart account solution for [CosmWasm][1]-enabled chains

## How does this work

Our goal is to make the SCA can be considered as the EOA with some extra features

In order to achieve this, the SCA must implement [execute and query methods][2], `after_execute` and `validate`:

```rust
// execute method
pub struct AfterExecute {
    pub msgs: Vec<MsgData>
}

// query method
pub struct Validate { 
    pub msgs: Vec<MsgData>
}
```

The state machine will call `validate` right before a tx is about going to mempool. And `after_execute` will be called by the `authentication message` requested to include in the tx, which will be final and executed after all other messages have finished executing.

- In `validate`, the SCA is provided with details of the tx. It can do some basic checks here that not requiring a state updation.

- In `after_execute`, The SCA is provided with detailed information about the tx and can access the results of the tx execution. It can perform checking logic, updating account state, etc. And finally determine if the transaction is successful or not

## Demo

This repository contains two SCAs for demo purpose. Note, they are not considered ready for production use:

| Contract                                               | Description                                     |
| ------------------------------------------------------ | ----------------------------------------------- |
| [`account-base`](./contracts/base/)                    | base account with required function             |
| [`account-spend-limit`](./contracts/spend-limit/)      | account with spend limit checking               |


## License

TBD

[1]: https://cosmwasm.com/
[2]: https://github.com/aura-nw/smart-account-sample/packages/smart-account/src/lib.rs#L24-L35