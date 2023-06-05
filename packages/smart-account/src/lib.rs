use cosmwasm_schema::cw_serde;

// A data structure representing an account's message 
// will be passed into the call to the smart-account contract every time tx arrives
#[cw_serde]
pub struct MsgData {
    pub type_url: String, // url type of message
    pub value:    String, // value of message
    // etc
    //  MsgData {
    //      type_url: "/cosmos.bank.v1beta1.MsgSend",
    //      value: "{fromAddress:\"aura172r4c7mng5y6ccfqp5klwyulshx6dh2mmd2r0xnmsgugaa754kws8u96pq\",toAddress:\"aura1y3u4ht0p69gz757myr3l0fttchhw3fj2gpeznd\",amount:[{denom:\"uaura\",amount:\"200\"}]}"
    //  }
}

/// Any contract must implement these below execute and query function in order to
/// qualify as an smart account.


// method that allow smart-account verify and check tx after it executed
// Also perform logic to update its state
// must implement in `execute` entry_point
#[cw_serde]
pub struct AfterExecute {
    //list of messages in transaction 
    pub msgs: Vec<MsgData>
}

// method that allow smart-account perform some basic check on tx before it going to mempool
// must implement in `query` entry_point
#[cw_serde]
pub struct Validate {
    //list of messages in transaction 
    pub msgs: Vec<MsgData>
}
