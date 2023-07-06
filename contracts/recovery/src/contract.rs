#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, from_binary};
use cw2::set_contract_version;
use sha2::{Digest, Sha256};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, SudoMsg, InstantiateMsg, QueryMsg, Credentials};
use crate::state::RECOVER_KEY;
use smart_account::{AfterExecute, PreExecute, Recover, MsgData};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:recovery";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Handling contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let recover_key = hex::decode(msg.recover_key).unwrap();
    
    // set recover key for this contract 
    RECOVER_KEY.save(deps.storage, &recover_key)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
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

#[entry_point]
pub fn sudo(deps: DepsMut, env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::Recover(Recover{ pub_key, credentials, ..}) => sudo_recover(deps,env,pub_key,credentials)
    }
}

fn sudo_recover(
    deps: DepsMut,
    _env: Env,
    pub_key: Binary,
    credentials: Binary,
) -> Result<Response, ContractError> {

    // this is method of sudo entry point, so no need for external users checking here

    let key_bytes_hash = sha256(&pub_key);
    
    let recover_key = RECOVER_KEY.load(deps.storage)?;
    
    let credentials: Credentials = from_binary(&credentials)?;

    if !deps.api.secp256k1_verify(&key_bytes_hash, &credentials.signature, &recover_key).unwrap() {
        return Err(ContractError::CustomError { val: "Invalid signature for recovery".to_string() });
    }

    Ok(Response::new().add_attribute("action", "recover"))
}

pub fn sha256(msg: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(msg);
    hasher.finalize().to_vec()
}

