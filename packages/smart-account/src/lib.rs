use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Binary};

// A data structure representing an account's message 
// will be passed into the call to the smart-account contract every time tx arrives
#[cw_serde]
pub struct Any {
    pub type_url: String, // url type of message
    pub value:    Binary, // value of message
    // etc.
    //  MsgData {
    //      type_url: "/cosmos.bank.v1beta1.MsgSend",
    //      value: "{fromAddress:\"aura172r4c7mng5y6ccfqp5klwyulshx6dh2mmd2r0xnmsgugaa754kws8u96pq\",toAddress:\"aura1y3u4ht0p69gz757myr3l0fttchhw3fj2gpeznd\",amount:[{denom:\"uaura\",amount:\"200\"}]}"
    //  }
}

// fee information
#[cw_serde]
pub struct CallInfo {
    pub fee: Vec<Coin>,
    pub gas: u64,
    pub fee_payer: String,
    pub fee_granter: String,
}

/// Any contract must implement these below sudo methods in order to
/// qualify as an smart account.


// execute method that allow smart-account verify and check tx after it executed
// Also perform logic to update its state
#[cw_serde]
pub struct AfterExecute {
    //list of messages in transaction 
    pub msgs: Vec<Any>,
    // fee information of transaction
    pub call_info: CallInfo,
    // Is tx executed throught authz msg
    pub is_authz: bool
}

// execute method that allow smart-account perform some basic check on tx before it going to mempool
#[cw_serde]
pub struct PreExecute {
    //list of messages in transaction 
    pub msgs: Vec<Any>,
    // fee information of transaction
    pub call_info: CallInfo,
    // Is tx executed throught authz msg
    pub is_authz: bool
}


// sudo method that activate the smart account recovery function
#[cw_serde]
pub struct Recover {
    // signer of tx
    pub caller: String,
    // new public key for smartaccount
    pub pub_key: Binary,
    // authorization
    pub credentials: Binary,
}