#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::OWNER;
use smart_account::{AfterExecute, PreExecute, MsgData};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:base";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Handling contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // set owner for this contract for future admin update
    OWNER.save(deps.storage, &info.sender)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    match msg {
    }
}

/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AfterExecute(AfterExecute{ msgs })
        => execute_after_execute(deps,env,info,msgs),

        ExecuteMsg::PreExecute(PreExecute{ msgs })
        => execute_pre_execute(deps,env,info,msgs)
    }
}

fn execute_after_execute(
    _deps: DepsMut,
    env: Env,
    info: MessageInfo,
    _msgs: Vec<MsgData>,
) -> Result<Response, ContractError> {

    // only smart account can execute this function
    // must implement this check to make sure, no one other than itself can execute smart account logic
    if info.sender != env.contract.address {
        return Err(ContractError::Unauthorized {});
    }
    
    // verify, check, upadte ... logic here

    Ok(Response::new().add_attribute("action", "after_execute"))
}

fn execute_pre_execute(
    _deps: DepsMut,
    env: Env,
    info: MessageInfo,
    _msgs: Vec<MsgData>,
) -> Result<Response, ContractError> {

    // only smart account can execute this function
    // must implement this check to make sure, no one other than itself can execute smart account logic
    if info.sender != env.contract.address {
        return Err(ContractError::Unauthorized {});
    }
    
    // verify, check, upadte ... logic here

    Ok(Response::new().add_attribute("action", "pre_execute"))
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
    }
}
