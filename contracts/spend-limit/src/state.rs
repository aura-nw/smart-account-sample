use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Uint128, Addr, Timestamp};
use cw_storage_plus::{Map,Item};

#[cw_serde]
pub struct Limit {
    pub limit: Uint128, // the limit amount of coin
    pub used: Uint128, // the current amount of coins used
    pub time_set: Timestamp, // time that set limit
}

pub const LIMITS: Map<String, Limit> = Map::new("limits");

pub const OWNER: Item<Addr> = Item::new("owner");