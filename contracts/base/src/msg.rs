use cosmwasm_schema::{cw_serde, QueryResponses};
use smart_account::{AfterExecute, PreExecute};

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum SudoMsg {
    // required `AfterExecute` method
    AfterExecute(AfterExecute),

    // required `PreExecute` method
    PreExecute(PreExecute),
}

/// Message type for `migrate` entry_point
#[cw_serde]
pub enum MigrateMsg {}


/// Message type for `query` entry_point
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
}

