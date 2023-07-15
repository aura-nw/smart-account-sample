import * as dotenv from "dotenv"
import { stringToPath } from "@cosmjs/crypto"
import { DirectSecp256k1HdWallet } from "@cosmjs/proto-signing"
import { SigningStargateClient, GasPrice, calculateFee } from "@cosmjs/stargate"
import { TxRaw } from "cosmjs-types/cosmos/tx/v1beta1/tx.js"
import { toUtf8 } from "@cosmjs/encoding"
import { assert } from "@cosmjs/utils"
import { SmartAccount } from "@aura-nw/aurajs/main/codegen/smartaccount/account.js"
import { MsgExecuteContract } from "cosmjs-types/cosmwasm/wasm/v1/tx.js";

dotenv.config()

assert(process.env.MNEMONIC, "MNEMONIC must be set")
const mnemonic = process.env.MNEMONIC

assert(process.env.PREFIX, "PREFIX must be set")
const prefix = process.env.PREFIX

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

async function connectSigner() {
    const path = stringToPath("m/44'/118'/0'/0/0")
    const wallet = await DirectSecp256k1HdWallet.fromMnemonic(mnemonic, {hdPaths:[path], "prefix":prefix})
    const [firstAccount] = await wallet.getAccounts()
    const client = await SigningStargateClient.connectWithSigner(endpoint, wallet, {prefix,gasPrice})

    const mneAddr = firstAccount.address
    return {client, mneAddr}
}

async function getSignData(client) {
    const queryClient = client.getQueryClient()
    const accountRaw = await queryClient.auth.account(accountAddr)
    const account = SmartAccount.decode(accountRaw.value)

    const nextSignData = {
        chainId: await client.getChainId(),
        accountNumber: parseInt(account.accountNumber),
        sequence: parseInt(account.sequence),
    };
    console.log(`Sign data set to: ${JSON.stringify(nextSignData)}`)

    return nextSignData
}


async function main(mintAddress, tokenId) {
    const {client, mneAddr} = await connectSigner()

    const mintMsg = {
        typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
        value: {
            sender: accountAddr,
            contract: mintAddress,
            msg: toUtf8(JSON.stringify(
            {
              mint: {
                "token_id": tokenId,
                "owner": accountAddr
              }
            })),
            "funds": []
        }
    }



    const memo = "";
    const signData = await getSignData(client)

    const usedFee = calculateFee(gasWanted, gasPrice)
    
    client.registry.register("/cosmwasm.wasm.v1.MsgExecuteContract", MsgExecuteContract)

    const signed = await client.sign(mneAddr, [mintMsg], usedFee, memo, signData)

    const tx = Uint8Array.from(TxRaw.encode(signed).finish())
  
    const p = await client.broadcastTx(tx)

    console.log(p)
}

const args = process.argv.slice(1)
console.log(args)
console.log(args.length)
assert(args.length == 2, "Usage: node nft.js <cw721_address> <token_id>")
await main(args[1], args[2])
