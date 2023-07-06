use cosmwasm_schema::{cw_serde, QueryResponses};
use smart_account::{AfterExecute, PreExecute, Recover};

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {
    pub recover_key: String,
}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {
    // required `AfterExecute` method
    AfterExecute(AfterExecute),

    // required `PreExecute` method
    PreExecute(PreExecute),
}

#[cw_serde]
pub enum SudoMsg {
    Recover(Recover),
}

/// Message type for `migrate` entry_point
#[cw_serde]
pub enum MigrateMsg {}


/// Message type for `query` entry_point
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
}

#[cw_serde]
pub struct Credentials {
    pub signature: Vec<u8>
}
