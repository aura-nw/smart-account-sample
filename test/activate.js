import * as dotenv from "dotenv"
import { stringToPath } from "@cosmjs/crypto"
import { DirectSecp256k1HdWallet } from "@cosmjs/proto-signing"
import { SigningStargateClient, GasPrice, calculateFee } from "@cosmjs/stargate"
import { TxRaw } from "cosmjs-types/cosmos/tx/v1beta1/tx.js"
import { toUtf8, fromBase64 } from "@cosmjs/encoding"
import { assert } from "@cosmjs/utils"
import { SmartAccount } from "@aura-nw/aurajs/main/codegen/aura/smartaccount/account.js"
import { MsgActivateAccount } from "@aura-nw/aurajs/main/codegen/aura/smartaccount/tx.js"
import { PubKey as CosmosCryptoSecp256k1Pubkey } from "cosmjs-types/cosmos/crypto/secp256k1/keys.js";

dotenv.config()

// Required env vars
assert(process.env.MNEMONIC, "MNEMONIC must be set")
const mnemonic = process.env.MNEMONIC

assert(process.env.PREFIX, "PREFIX must be set")
const prefix = process.env.PREFIX

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


async function main(codeId, salt, pubKey, initMsg) {
  const {client, mneAddr} = await connectSigner()

  pubKey = JSON.parse(pubKey)
  
  const activateAccountMsg = {
      typeUrl: "/aura.smartaccount.v1beta1.MsgActivateAccount",
      value: {
        accountAddress: accountAddr,
        codeId: codeId,
        salt: toUtf8(salt),
        pubKey: {
          typeUrl: pubKey["@type"],
          value:   Uint8Array.from(
            CosmosCryptoSecp256k1Pubkey.encode(
              CosmosCryptoSecp256k1Pubkey.fromPartial({
                key: fromBase64(pubKey["key"]),
              }),
            ).finish(),
          )
        },
        initMsg: toUtf8(initMsg)
    }
  }

  const memo = "";
  const signData = await getSignData(client)

  const usedFee = calculateFee(gasWanted, gasPrice)

  client.registry.register("/aura.smartaccount.v1beta1.MsgActivateAccount", MsgActivateAccount)
  
  const signed = await client.sign(mneAddr, [activateAccountMsg], usedFee, memo, signData)

  const tx = Uint8Array.from(TxRaw.encode(signed).finish())
  
  const p = await client.broadcastTx(tx)

  console.log(p)
}

const args = process.argv.slice(2)
assert(args.length == 4, "Usage: node activate.js <code_id> <salt> <pub_key> <init_msg>")
await main(args[0], args[1], args[2], args[3])