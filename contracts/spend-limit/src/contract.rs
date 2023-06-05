#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary, Uint128, Storage, Timestamp};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, MsgSend};
use crate::state::{
    LIMITS, Limit,
    OWNER
};

use smart_account::{AfterExecute, Validate, MsgData};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:iaccount";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const SECOND_PER_HOUR: u64 = 3600;
// Cosmos bank send message url type
const MSG_BANK_SEND: &str = "/cosmos.bank.v1beta1.MsgSend";

/// Handling contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    OWNER.save(deps.storage, &info.sender)?;

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
        
        ExecuteMsg::SetSpendLimit { denom, amount }
        => execute_set_spend_limit(deps, env, info, denom, amount),

        ExecuteMsg::AfterExecute(AfterExecute{ msgs })
        => execute_after_execute(deps,env,info,msgs)
    }
}

fn execute_set_spend_limit(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    denom: String,
    amount: Uint128
) -> Result<Response, ContractError> {
    // only onwer can execute this function
    let owner = OWNER.load(deps.storage)?;
    if info.sender != owner {
        return Err(ContractError::Unauthorized {});
    }

    // new limitation for coin using
    let limit: Limit = Limit { 
        limit: amount, 
        used: Uint128::zero(),
        time_set: env.block.time, 
    };

    LIMITS.save(deps.storage, denom.clone(), &limit)?;

    Ok(Response::new().add_attribute("action", "set_spend_limit")
        .add_attribute("denom", denom)
        .add_attribute("amount", amount))
}

fn check_limit(storage: &dyn Storage, denom: String, amount: Uint128, block_time: Timestamp) -> Result<bool, ContractError> {
    
    if let Some(limit) = LIMITS.may_load(storage, denom)? {
        // spend limit only available in one hour
        if block_time.minus_seconds(SECOND_PER_HOUR) > limit.time_set {
            return Ok(true)
        }
        
        // check if amount of coin used reach limit
        if limit.limit < amount || limit.limit.checked_sub(amount).unwrap() < limit.used {
            return Ok(false)
        }
    }

    Ok(true)
}

fn execute_after_execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msgs: Vec<MsgData>,
) -> Result<Response, ContractError> {
    
    // only smart account can execute this function
    if info.sender != env.contract.address {
        return Err(ContractError::Unauthorized {});
    }

    for msg in msgs {
        if msg.type_url == String::from(MSG_BANK_SEND) { // if message is bank send message
            let msg_send: MsgSend = serde_json_wasm::from_str(&msg.value)
            .map_err(|_| ContractError::CustomError{val: String::from("Invalid MsgSend message format!")})?;
	
		    for coin in msg_send.amount {
                // check spend limit
                if !check_limit(deps.storage, coin.denom.clone(), coin.amount, env.block.time)? {
                    return Err(ContractError::CustomError { 
                        val: format!("limit exceed for denom: {}",coin.denom) 
                    })
                }
                
                // update used coin
                if let Some(mut limit) = LIMITS.may_load(deps.storage, coin.denom.clone())? {
                    limit.used = limit.used.checked_add(coin.amount).unwrap();
    
                    LIMITS.save(deps.storage, coin.denom, &limit)?;
                }
            }
        }
    }

    Ok(Response::new().add_attribute("action", "after_execute"))
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Validate(Validate{msgs})
        => to_binary(&query_validate(_deps,_env,msgs)?),
    }
}

fn query_validate(
    _deps: Deps,
    _env: Env,
    msgs: Vec<MsgData>
) -> StdResult<bool> {

    // only allow Bank::Send messages
    for msg in msgs {
        if msg.type_url != String::from(MSG_BANK_SEND) {
            return Err(cosmwasm_std::StdError::GenericErr {msg: "invalid message type".to_string()})
        }
    }

    Ok(true)
}
