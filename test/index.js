import * as dotenv from "dotenv"
import { SigningCosmWasmClient, Secp256k1HdWallet } from "cosmwasm"
import { stringToPath } from "@cosmjs/crypto"
import { calculateFee, GasPrice } from "@cosmjs/stargate"
import { TxRaw } from "cosmjs-types/cosmos/tx/v1beta1/tx.js"
import { assert } from "@cosmjs/utils"
import { toUtf8 } from "@cosmjs/encoding"
import chalk from "chalk"

dotenv.config()

// Required env vars
assert(process.env.MNEMONIC, "MNEMONIC must be set")
const mnemonic = process.env.MNEMONIC

assert(process.env.PREFIX, "PREFIX must be set")
const prefix = process.env.PREFIX

// The fee denom
assert(process.env.DENOM, "DENOM must be set")
const denom = process.env.DENOM

assert(process.env.ADDRESS, "ADDRESS must be set")
const accountAddr = process.env.ADDRESS

assert(process.env.ENDPOINT, "ENDPOINT must be set")
const endpoint = process.env.ENDPOINT

assert(process.env.GAS_PRICE, "GAS_PRICE must be set. E.g. '0.025ueaura'")
const gasPrice = GasPrice.fromString(process.env.GAS_PRICE)

assert(process.env.GAS_WANTED, "GAS_WANTED must be set")
const gasWanted = parseInt(process.env.GAS_WANTED)

const successColor = chalk.green
const infoColor = chalk.gray

const args = process.argv.slice(2)
assert(args.length == 4, "must be 'from_address', 'funds', 'account_number' and 'sequence'")

export async function connectWallet() {
    // Create a wallet
    const path = stringToPath("m/44'/118'/0'/0/0");
    const wallet = await Secp256k1HdWallet.fromMnemonic(mnemonic, 
            {hdPaths:[path], "prefix":prefix})
    const [firstAccount] = await wallet.getAccounts()
    const client = await SigningCosmWasmClient.connectWithSigner(endpoint, wallet, {
    prefix,
    gasPrice,
    });
    const botAddress = firstAccount.address

    console.log("\n------------------------------------------------------------------------------------")
    console.log(successColor("SigningCosmWasmClient CONNECTION Success"))

    return {client, botAddress}
}

let nextSignData = {
    chainId: "",
    accountNumber: 0,
    sequence: 0,
};

export async function resetSignData(client, botAddress) {
    nextSignData = {
      chainId: await client.getChainId(),
      accountNumber: parseInt(args[2]),
      sequence: parseInt(args[3]),
    };
    console.log(infoColor(`Sign data set to: ${JSON.stringify(nextSignData)}`))
}

async function main() {
    const {client, botAddress} = await connectWallet() // connect to wallet with mnemonic 
    
    const sendMsg = {
        typeUrl: "/cosmos.bank.v1beta1.MsgSend",
        value: {
          fromAddress: accountAddr,
          toAddress: args[0],
          amount: [{denom:denom,amount:args[1]}],
        }
    }
    
    const validateMsg = {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: {
        sender: accountAddr,
        contract: accountAddr,
        msg: toUtf8(JSON.stringify(
          {after_execute: {
              msgs: [
                {
                  type_url: "/cosmos.bank.v1beta1.MsgSend",
                  value: JSON.stringify({
                    from_address: accountAddr,
                    to_address: args[0],
                    amount: [{denom:denom,amount:args[1]}] 
                  })
                }
              ]
            }
          })),
        "funds": []
      }
    }
    
    const memo = "test memo";

    let usedFee = calculateFee(gasWanted, gasPrice)
    
    await resetSignData(client, accountAddr)
    const signData = nextSignData

    const signed = await client.sign(botAddress, [sendMsg, validateMsg], usedFee, memo, signData)
    const tx = Uint8Array.from(TxRaw.encode(signed).finish())

    const p1 = await client.broadcastTx(tx)

    console.log(p1)
}

main().then(
    () => {
      console.info("Done")
      process.exit(0)
    },
    (error) => {
      console.error(error)
      process.exit(1)
    },
);
