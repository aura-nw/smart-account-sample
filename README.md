# smart-account-sample

a [smart account][4] solution for [CosmWasm][1]-enabled chains

## How does this work

Our goal is to make the SCA can be considered as the EOA with some extra features

In order to achieve this, the SCA must implement [sudo methods][2], `after_execute` and `pre_execute`:

```rust
// sudo method
pub struct AfterExecute {
    //list of messages in transaction 
    pub msgs: Vec<Any>,
    // fee information of transaction
    pub call_info: CallInfo,
    // Is tx executed throught authz msg
    pub is_authz: bool
}

// sudo method
pub struct PreExecute { 
    //list of messages in transaction 
    pub msgs: Vec<Any>,
    // fee information of transaction
    pub call_info: CallInfo,
    // Is tx executed throught authz msg
    pub is_authz: bool
}
```

The state machine will call `pre_execute` right before a tx is about going to mempool. And `after_execute` will be called by the `validate message` which is requested to include in the tx, this message will be final and executed after all other messages have finished executing.

- In `pre_execute`, the SCA is provided with details of the tx. It can perform checking logic, updating account state, etc. And determine if the transaction is allowed to enter the mempool or not?

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
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.13.0
```
This command will generate 3 file `base.wasm`, `spend_limit.wasm` and `recovery.wasm` in artifacts folder

</br>

### II. Run a LocalNet

</br>

**Prerequisite**
- Go 1.19
- Ignite v0.22.1

</br>

```
git clone https://github.com/aura-nw/aura.git --branch serenity

cd aura

// change "whitelist_code_id" genesis state in config.yml file (test only)
// whitelist_code_id: [
//  {
//      "code_id": "1",
//      "status": true    
//  },
//  {
//      "code_id": "2",
//      "status": true
//  }
//]

make build

ignite chain serve -v
```

</br>

**Setup test env**
```
cd ./test

// change 'XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX' in .npmrc file using github access token with read package privilege

npm install
```

Generate a new mnemonic. For example:
```
MNEMONIC `"deputy cousin control dentist cost rich mention stomach rabbit amazing glove gain lend sign bronze mushroom task wedding captain add script wrestle repair camp"`
```
change .env file with that mnemonic

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

**Submit whitelist proposal to Serenity testnet (optional)**

In case you want to deploy smart account contract to Aura Serenity testnet, please create a proposal file with content:
```
{
    "title": "SmartAccount Param Change",
    "description": "Update whitelist code_id",
    "changes": [
      {
        "subspace": "smartaccount",
        "key": "WhitelistCodeID",
        "value": [
            {
                "code_id": "<your-code-id>",
                "status": true
            }
        ]
      }
    ],
    "deposit": "1000000uaura"
 }
```
and run bellow command to submit it:

```
aurad tx gov submit-proposal param-change <your-proposal-file> --from <your-key> --chain-id serenity-testnet-001 --fees 300uaura --yes
```

After that, please ping our admins so that we can vote for it.

**Generate predictable account address**
```
export CODE_ID=1
export INIT_MSG='{"owner":"'$(aurad keys show $SIGNER -a)'"}'
export PUBKEY='{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"AnZfdXVALfIcNjpqgzH/4nWsSpP7l5PiCyZAuAWQRBUz"}'
export SALT="account1"

aurad q smartaccount generate-account \
    $CODE_ID \
    $SALT \
    $INIT_MSG \
    $PUBKEY 

// output: SPENDLIMIT_CONTRACT_ADDR
```
change .env file with ADDRESS `"SPENDLIMIT_CONTRACT_ADDR"`

</br>

**Send fund to account**
```
export ACCOUNT_ADDR=<SPENDLIMIT_CONTRACT_ADDR>

aurad tx bank send $(aurad keys show $SIGNER -a) $ACCOUNT_ADDR 10000000uaura \
    --from $SIGNER \
    --fees 200uaura \
    --chain-id $CHAIN_ID
```

</br>

**Activate smart account**
```
node activate.js $CODE_ID $SALT $PUBKEY $INIT_MSG
```

</br>

**Set spend-limit**
```
aurad tx wasm execute $ACCOUNT_ADDR\
    '{"set_spend_limit":{"denom":"uaura","amount":"10000"}}' \
    --from $SIGNER \
    --gas-prices 0.025uaura \
    --chain-id $CHAIN_ID \
    --gas=auto \
    --gas-adjustment 1.3
```

</br>

### IV. Test

**Send token from smart-account success**
```
export TO_ADDRESS=<ANY_ADDRESS>
// denom is uaura
export AMOUNT=5000

node send.js $TO_ADDRESS $AMOUNT
```

</br>

**Reach spend-limit, transaction fail**
```
export AMOUNT=5001

node send.js $TO_ADDRESS $AMOUNT
```

## V. Recover Test
**Create Recovery Account**

The creation process is as above with the parameters
```
export CODE_ID=<CODE_ID_OF_RECOVERY_CONTRACT>
export INIT_MSG='{"recover_key":"024ab33b4f0808eba493ac4e3ead798c8339e2fd216b20ca110001fd094784c07f"}'
export PUBKEY='{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"AnZfdXVALfIcNjpqgzH/4nWsSpP7l5PiCyZAuAWQRBUz"}'
export SALT="account1"

// output: RERCOVERY_CONTRACT_ADDR
```
change .env file with ADDRESS `"RERCOVERY_CONTRACT_ADDR"`


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

**Create and send fund to Signer**
```
aurad keys add Signer

aurad tx bank send $(aurad keys show Cantho -a) $(aurad keys show Signer -a) 10000000uaura \
    --from Cantho \
    --fees 200uaura \
    --chain-id $CHAIN_ID
```

</br>

**Recover Account Pubkey**
```
export ACCOUNT_ADDR=<RECOVERY_CONTRACT_ADDRR>
export NEW_PUBKEY='{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"A/2t0ru/iZ4HoiX0DkTuMy9rC2mMeXmiN6luM3pa+IvT"}'
export CREDENTIALS="eyJzaWduYXR1cmUiOls4LDI0NywxOTksMTM4LDIzOCwxOTQsMTI5LDI1NCwyNTEsMTMxLDIzNywyNDEsMzMsODcsMTAzLDQyLDEzOCwyMjcsMjM3LDEyMyw5MiwyMjYsNjMsMTc0LDIwMSw2OCwyMSwzMiw5OSwxMzEsMjM1LDIzMSwyOCwxNzAsMjAzLDE4MCwxMTEsMiwyMjAsMTI2LDE0NCwxNzQsMTYxLDkyLDI1LDIwMiw2MiwxODEsMjUyLDE3OCwxNjMsNDAsMTc3LDIxMCwxNzYsNSwxNDUsMjAwLDU0LDE5MiwxMDgsMyw3Nyw2MV19"

aurad tx smartaccount recover $ACCOUNT_ADDR $NEW_PUBKEY $CREDENTIALS \
    --from Signer \
    --chain-id $CHAIN_ID \
    --gas=auto \
    --gas-adjustment 1.4  \
    --gas-prices 0.025uaura \
    --broadcast-mode=block
```

- Generate `credentials`:
    ```Javascript
    import {Secp256k1, sha256, EnglishMnemonic, Bip39, Slip10, Slip10Curve, stringToPath} from "@cosmjs/crypto"
    import { fromBase64 } from "@cosmjs/encoding"

    const recover_mnemonic = "fat history among correct tribe face armed rough language wonder era ribbon puppy car subject cube provide video math address simple skate swap oval"
    const hdPath = stringToPath("m/44'/118'/0'/0/0")
    const mnemonicChecked = new EnglishMnemonic(recover_mnemonic)
    const seed = await Bip39.mnemonicToSeed(mnemonicChecked, "")
    const { privkey } = Slip10.derivePath(Slip10Curve.Secp256k1, seed, hdPath)
    let { pubkey } = await Secp256k1.makeKeypair(privkey)
    pubkey = Secp256k1.compressPubkey(pubkey)

    let new_pubkey = "A/2t0ru/iZ4HoiX0DkTuMy9rC2mMeXmiN6luM3pa+IvT"
    const hashedPubkey = sha256(fromBase64(new_pubkey))
    const signaturePubkey = await Secp256k1.createSignature(hashedPubkey, privkey)
    const signaturePubkeyBytes = [...signaturePubkey.r(32), ...signaturePubkey.s(32)]

    let credentials = btoa(JSON.stringify({
        signature: signaturePubkeyBytes
    }))
    ```

</br>

change .env file with MNEMONIC `"era attitude lucky six physical elite melt industry space motion quit shallow under dust present cross heavy wrist sweet total gravity duck twist wine"` then we already to go.


[1]: https://cosmwasm.com/
[2]: https://github.com/aura-nw/smart-account-sample/packages/smart-account/src/lib.rs#L24-L36
[3]: https://github.com/aura-nw/smart-account-sample/packages/smart-account/src/lib.rs#L44-L54
[4]: https://aura-network.notion.site/Smart-Account-e69e51d6449b46dcb7c157a325dfb44f
