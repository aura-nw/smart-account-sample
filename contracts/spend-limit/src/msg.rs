use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Uint128, Coin};
use smart_account::{AfterExecute, PreExecute};

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String
}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {
    // owner set spend limit for smart-account
    SetSpendLimit {
        denom: String, // denom string etc. "uaura"
        amount: Uint128 // amount string etc. "10000"
    },

    // required `PreExecute` method
    PreExecute(PreExecute),

    // required `AfterExecute` method
    AfterExecute(AfterExecute),
}

/// Message type for `migrate` entry_point
#[cw_serde]
pub enum MigrateMsg {}

// struct for message with type `/cosmos.bank.v1beta1.MsgSend`
// it's same as message struct in cosmos-sdk or cosmjs
#[cw_serde]
pub struct MsgSend {
    pub from_address: String, // sender
    pub to_address: String, // receiver
    pub amount: Vec<Coin>, // amount
}

/// Message type for `query` entry_point
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
}

