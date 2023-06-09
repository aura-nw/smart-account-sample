# smart-account-sample

a [smart account][4] solution for [CosmWasm][1]-enabled chains

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

The state machine will call `validate` right before a tx is about going to mempool. And `after_execute` will be called by the `authentication message` which is requested to include in the tx, this message will be final and executed after all other messages have finished executing.

- In `validate`, the SCA is provided with details of the tx. It can do some basic checks here without requiring a state update. And determine if the transaction is allowed to enter the mempool?

- In `after_execute`, The SCA is provided with detailed information about the tx and can access the results of the tx execution. It can perform checking logic, updating account state, etc. And finally determine if the transaction is successful or not?

</br>

Optional [sudo method][3] `recover` that activate the smart account recovery function  
```rust
// sudo method
pub struct Recover {
    pub caller: String,
    pub pub_key: Binary,
    pub credentials: Binary,
}
```

The state machine will call the `recover` method when the **recover message** is processed in the SmartAccountModule, then update the pubkey of account if the recovery check is successful.  

## Demo

This repository contains three SCAs for demo purpose. Note, they are not considered ready for production use:

| Contract                                               | Description                                     |
| ------------------------------------------------------ | ----------------------------------------------- |
| [`account-base`](./contracts/base/)                    | base account with required methods              |
| [`account-recovery`](./contracts/recovery/)            | account with recovery enabled                   |
| [`account-spend-limit`](./contracts/spend-limit/)      | account with spend limit checking               |

### I. Build Project

</br>

```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.11
```
This command will generate 3 file `base.wasm`, `spend_limit.wasm` and `recovery.wasm` in artifacts folder

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
exporZLt ACCOUNT_ADDR=<SMART_CONTRACT_ADDR>

aurad tx bank send $(aurad keys show $SIGNER -a) $ACCOUNT_ADDR 10000000uaura \
    --from $SIGNER \
    --fees 200uaura \
    --chain-id $CHAIN_ID
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
export ACCOUNT_NUMBER=10
export ACCOUNT_SEQUENCE=0
export TO_ADDRESS=<ANY_ADDRESS>
export AMOUNT=5000

node index.js $TO_ADDRESS $AMOUNT $ACCOUNT_NUMBER $ACCOUNT_SEQUENCE
```

</br>

**Reach spend-limit, transaction fail**
```
export ACCOUNT_SEQUENCE=1
export AMOUNT=5001

node index.js $TO_ADDRESS $AMOUNT $ACCOUNT_NUMBER $ACCOUNT_SEQUENCE
```

## V. Recover Test
**Instantiate Recovery Account**
```
export CODE_ID=<CODE_ID_OF_RECOVERY_CONTRACT>
export INIT_MSG='{"recover_key":"024ab33b4f0808eba493ac4e3ead798c8339e2fd216b20ca110001fd094784c07f"}'
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

- Generate `recovery key`
    ```Javascript
    import {Secp256k1, sha256, EnglishMnemonic, Bip39, Slip10, Slip10Curve, stringToPath} from "@cosmjs/crypto"

    const mnemonic = "fat history among correct tribe face armed rough language wonder era ribbon puppy car subject cube provide video math address simple skate swap oval"
    const hdPath = stringToPath("m/44'/118'/0'/0/0")
    const mnemonicChecked = new EnglishMnemonic(mnemonic)
    const seed = await Bip39.mnemonicToSeed(mnemonicChecked, "")
    const { privkey } = Slip10.derivePath(Slip10Curve.Secp256k1, seed, hdPath)
    let { pubkey } = await Secp256k1.makeKeypair(privkey)

    let recover_key = Secp256k1.compressPubkey(pubkey)
    ```

</br>

**Recover Account Pubkey**
```
export $ACCOUNT_ADDR=<RECOVERY_CONTRACT_ADDRR>
export $NEW_PUBKEY=03fdadd2bbbf899e07a225f40e44ee332f6b0b698c7979a237a96e337a5af88bd3
export $CREDENTIALS="eyJzaWduYXR1cmUiOls4LDI0NywxOTksMTM4LDIzOCwxOTQsMTI5LDI1NCwyNTEsMTMxLDIzNywyNDEsMzMsODcsMTAzLDQyLDEzOCwyMjcsMjM3LDEyMyw5MiwyMjYsNjMsMTc0LDIwMSw2OCwyMSwzMiw5OSwxMzEsMjM1LDIzMSwyOCwxNzAsMjAzLDE4MCwxMTEsMiwyMjAsMTI2LDE0NCwxNzQsMTYxLDkyLDI1LDIwMiw2MiwxODEsMjUyLDE3OCwxNjMsNDAsMTc3LDIxMCwxNzYsNSwxNDUsMjAwLDU0LDE5MiwxMDgsMyw3Nyw2MV19"

aurad tx smartaccount recover $ACCOUNT_ADDR $NEW_PUBKEY $CREDENTIALS \
    --from $SIGNER \
    --chain-id $CHAIN_ID \
    --gas=auto \
    --gas-adjustment 1.4  \
    --gas-prices 0.025uaura \
    --broadcast-mode=block
```

- Generate `credentials`:
    ```Javascript
    import {Secp256k1, sha256, EnglishMnemonic, Bip39, Slip10, Slip10Curve, stringToPath} from "@cosmjs/crypto"

    const recover_mnemonic = "fat history among correct tribe face armed rough language wonder era ribbon puppy car subject cube provide video math address simple skate swap oval"
    const hdPath = stringToPath("m/44'/118'/0'/0/0")
    const mnemonicChecked = new EnglishMnemonic(recover_mnemonic)
    const seed = await Bip39.mnemonicToSeed(mnemonicChecked, "")
    const { privkey } = Slip10.derivePath(Slip10Curve.Secp256k1, seed, hdPath)
    let { pubkey } = await Secp256k1.makeKeypair(privkey)
    pubkey = Secp256k1.compressPubkey(pubkey)

    let new_pubkey = "03fdadd2bbbf899e07a225f40e44ee332f6b0b698c7979a237a96e337a5af88bd3"
    const hashedPubkey = sha256(Uint8Array.from(Buffer.from(new_pubkey, 'hex')))
    const signaturePubkey = await Secp256k1.createSignature(hashedPubkey, privkey)
    const signaturePubkeyBytes = [...signaturePubkey.r(32), ...signaturePubkey.s(32)]

    let credentials = btoa(JSON.stringify({
        signature: signaturePubkeyBytes
    }))
    ```

</br>

change .env file with mnemonic `"era attitude lucky six physical elite melt industry space motion quit shallow under dust present cross heavy wrist sweet total gravity duck twist wine"` then we already to go.


[1]: https://cosmwasm.com/
[2]: https://github.com/aura-nw/smart-account-sample/packages/smart-account/src/lib.rs#L24-L36
[3]: https://github.com/aura-nw/smart-account-sample/packages/smart-account/src/lib.rs#L44-L54
[4]: https://aura-network.notion.site/Smart-Account-e69e51d6449b46dcb7c157a325dfb44f
