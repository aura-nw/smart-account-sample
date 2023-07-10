import {Bip39, Random, Slip10, Secp256k1, sha256, ripemd160, stringToPath, Slip10Curve} from "@cosmjs/crypto"
import { toBase64, toBech32 } from "@cosmjs/encoding"

const seed = Random.getBytes(32)
const hdPath = stringToPath("m/44'/118'/0'/0/0")
const { privkey } = Slip10.derivePath(Slip10Curve.Secp256k1, seed, hdPath)
let { pubkey } = await Secp256k1.makeKeypair(privkey)
pubkey = Secp256k1.compressPubkey(pubkey)


const mnemonic = Bip39.encode(seed)
const address = toBech32("aura", ripemd160(sha256(pubkey)))
const anyPubkey = {
    "@type": "/cosmos.crypto.secp256k1.PubKey",
    "key": toBase64(pubkey)
}

console.log("mnemonic: %s", mnemonic.data)
console.log("address: %s", address)
console.log("pubkey: %s", anyPubkey)
