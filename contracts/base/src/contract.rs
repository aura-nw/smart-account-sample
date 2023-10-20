#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{InstantiateMsg, MigrateMsg, QueryMsg, SudoMsg};
use crate::state::OWNER;
use smart_account::{AfterExecute, PreExecute, Any, CallInfo};

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

/// Handling contract sudo execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(
    deps: DepsMut, 
    env: Env, 
    msg: SudoMsg
) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::AfterExecute(AfterExecute{msgs, call_info, is_authz})
        => sudo_after_execute(deps,env,msgs,call_info,is_authz),

        SudoMsg::PreExecute(PreExecute{msgs, call_info, is_authz})
        => sudo_pre_execute(deps,env,msgs, call_info, is_authz)
    }
}

fn sudo_after_execute(
    _deps: DepsMut,
    _env: Env,
    _msgs: Vec<Any>,
    _call_info: CallInfo,
    _is_authz: bool,
) -> Result<Response, ContractError> {

    // verify, check, upadte ... logic here

    Ok(Response::new().add_attribute("action", "after_execute"))
}

fn sudo_pre_execute(
    _deps: DepsMut,
    _env: Env,
    _msgs: Vec<Any>,
    _call_info: CallInfo,
    _is_authz: bool,
) -> Result<Response, ContractError> {

    // verify, check, upadte ... logic here

    Ok(Response::new().add_attribute("action", "pre_execute"))
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
    }
}
