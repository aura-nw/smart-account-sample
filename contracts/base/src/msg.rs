use cosmwasm_schema::{cw_serde, QueryResponses};
use smart_account::{AfterExecute, Validate};

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {
    // required `AfterExecute` method
    AfterExecute(AfterExecute),
}

/// Message type for `migrate` entry_point
#[cw_serde]
pub enum MigrateMsg {}


/// Message type for `query` entry_point
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // required `Validate` method
    #[returns(bool)]
    Validate(Validate)
}

