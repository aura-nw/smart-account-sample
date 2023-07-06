use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;

// A data structure representing an account's message 
// will be passed into the call to the smart-account contract every time tx arrives
#[cw_serde]
pub struct MsgData {
    pub type_url: String, // url type of message
    pub value:    String, // value of message
    // etc.
    //  MsgData {
    //      type_url: "/cosmos.bank.v1beta1.MsgSend",
    //      value: "{fromAddress:\"aura172r4c7mng5y6ccfqp5klwyulshx6dh2mmd2r0xnmsgugaa754kws8u96pq\",toAddress:\"aura1y3u4ht0p69gz757myr3l0fttchhw3fj2gpeznd\",amount:[{denom:\"uaura\",amount:\"200\"}]}"
    //  }
}

/// Any contract must implement these below execute methods in order to
/// qualify as an smart account.


// execute method that allow smart-account verify and check tx after it executed
// Also perform logic to update its state
#[cw_serde]
pub struct AfterExecute {
    //list of messages in transaction 
    pub msgs: Vec<MsgData>
}

// execute method that allow smart-account perform some basic check on tx before it going to mempool
#[cw_serde]
pub struct PreExecute {
    //list of messages in transaction 
    pub msgs: Vec<MsgData>
}


// sudo method that activate the smart account recovery function
#[cw_serde]
pub struct Recover {
    pub caller: String,

    pub pub_key: Binary,

    pub credentials: Binary,
}