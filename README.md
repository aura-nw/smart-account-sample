# smart-account-sample

a [smart account][3] solution for [CosmWasm][1]-enabled chains

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

### I. Build Project

</br>

```
beaker wasm build
```
This command will generate 2 file `base.wasm` and `spend_limit.wasm` in artifacts folder

</br>

### II. Run a LocalNet

</br>

**Prerequisite**
- Go 1.18
- Ignite v0.22.1

</br>

```
git clone https://github.com/aura-nw/aura.git --branch smart-account-v0.2.0
cd aura
ignite chain serve -v
```

</br>

### III. Create a Smart-Account

</br>

**Store code**
```
export PATH_TO_WASM_FILE="./smart-account-sample/artifacts/spend_limit.wasm"
export SIGNER=Cantho
export CHAIN_ID=aura-testnet

aurad tx wasm store \
    $PATH_TO_WASM_FILE \
    --from $SIGNER \
    --chain-id $CHAIN_ID \
    --gas=auto \
    --gas-adjustment 1.4  \
    --gas-prices 0.025uaura \
    --broadcast-mode=block
```

</br>

**Create account**
```
export CODE_ID=1
export INIT_MSG='{}'
export PUBKEY="02765f7575402df21c363a6a8331ffe275ac4a93fb9793e20b2640b80590441533"
export SALT="salt"
export AMOUNT="0uaura"

aurad tx smartaccount create-account \
    $CODE_ID \
    $INIT_MSG \
    $PUBKEY \
    $SALT \
    --funds $AMOUNT \
    --from $SIGNER \
    --chain-id $CHAIN_ID \
    --gas=auto \
    --gas-adjustment 1.4  \
    --gas-prices 0.025uaura \
    --broadcast-mode=block
```

</br>

**Send fund to account**
```
export ACCOUNT_ADDR=<SMART_CONTRACT_ADDR>

aurad tx bank send $(aurad keys show $SIGNER -a) &ACCOUNT_ADDR 10000000uaura \
    --from $SIGNER \
    --fees 200uaura \
    --chain-id $CHAIN_ID

aurad tx bank send $(aurad keys show $SIGNER -a) aura1zg3rwaqyg933zxe9v5rcrdv755n28s7a3ypemz 1uaura \
    --from $SIGNER \
    --fees 200uaura \
    --chain-id $CHAIN_ID \
```

</br>

**Set spend-limit**
```
aurad tx wasm execute $ACCOUNT_ADDR\
    '{"set_spend_limit":{"denom":"uaura","amount":"10000"}}' \
    --from $SIGNER\
    --gas-prices 0.025uaura \
    --chain-id $CHAIN_ID \
    --gas=auto \
    --gas-adjustment 1.3
```

</br>

### IV. Test

**Setup test env**
```
cd ./test
npm install
```
change .env file

</br>

**Send token from smart-account success**
```
export TO_ADDRESS=<ANY_ADDRESS>
export AMOUNT=5000

node index.js $TO_ADDRESS $AMOUNT 
```

</br>

**Reach spend-limit, transaction fail**
```
export TO_ADDRESS=<ANY_ADDRESS>
export AMOUNT=5001

node index.js $TO_ADDRESS $AMOUNT 
```

## License

TBD

[1]: https://cosmwasm.com/
[2]: https://github.com/aura-nw/smart-account-sample/packages/smart-account/src/lib.rs#L24-L35
[3]: https://aura-network.notion.site/Smart-Account-e69e51d6449b46dcb7c157a325dfb44f