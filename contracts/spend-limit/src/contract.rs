#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, Timestamp,
    AllBalanceResponse, QueryRequest, BankQuery, QuerierWrapper, Empty,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{
    LIMITS, Limit,
    OWNER, BALANCES
};

use smart_account::{AfterExecute, MsgData, PreExecute};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:spend-limit";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const SECOND_PER_HOUR: u64 = 3600;

/// Handling contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let owner = deps.api.addr_validate(&msg.owner)?;

    OWNER.save(deps.storage, &owner)?;

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

        ExecuteMsg::PreExecute(PreExecute{ msgs })
        => execute_pre_execute(deps,env,info,msgs),

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

fn check_limit(limit: Limit, amount: Uint128, block_time: Timestamp) -> bool {
 
    // spend limit only available in one hour
    if block_time.minus_seconds(SECOND_PER_HOUR) > limit.time_set {
        return true
    }

    // check if amount of coin used reach limit
    if limit.limit < amount || limit.limit.checked_sub(amount).unwrap() < limit.used {
        return false
    }

    true
}

fn execute_after_execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    _msgs: Vec<MsgData>,
) -> Result<Response, ContractError> {

    // only smart account can execute this function
    if info.sender != env.contract.address {
        return Err(ContractError::Unauthorized {});
    }

    let pre_balances = BALANCES.load(deps.storage)?;

    // account after-execute tx balances
    let contract_balance = contract_all_balances(deps.querier,env.contract.address.to_string())?;
    let after_balances = contract_balance.amount;

    for pre_balance in pre_balances {
        // if has spendlimit for denom
        if let Some(mut limit) = LIMITS.may_load(deps.storage, pre_balance.denom.clone())? {
            let matching_coin = after_balances.iter().find(|fund| fund.denom.eq(&pre_balance.denom));
            let amount = match matching_coin {
                Some(coin) => coin.amount,
                None => Uint128::zero()
            };

            // check if coin with denom has been used
            // this for demo only, 
            // in real case, user can cheat here by including withdrawal message and deposit message in the same tx
            if pre_balance.amount > amount {
                // used amount
                let used_amount = pre_balance.amount.checked_sub(amount).unwrap();
                // check if spendlimit has been reach
                if !check_limit(limit.clone(), used_amount, env.block.time) {
                    return Err(ContractError::CustomError {
                        val: format!("limit exceed for denom: {}", pre_balance.denom)
                    })
                }

                // update used amount
                limit.used = limit.used.checked_add(used_amount).unwrap();
                LIMITS.save(deps.storage, pre_balance.denom, &limit)?;
            }
        }
    }

    Ok(Response::new().add_attribute("action", "after_execute"))
}

fn execute_pre_execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    _msgs: Vec<MsgData>,
) -> Result<Response, ContractError> {

    // only smart account can execute this function
    if info.sender != env.contract.address {
        return Err(ContractError::Unauthorized {});
    }

    // get the balances of contract
    let contract_balance = contract_all_balances(deps.querier, env.contract.address.to_string())?;

    // account pre-execute tx balances  
    BALANCES.save(deps.storage, &contract_balance.amount)?;

    Ok(Response::new().add_attribute("action", "pre_execute"))
}

fn contract_all_balances<'a>(querier: QuerierWrapper<'a, Empty>, address: String) -> StdResult<AllBalanceResponse> {
    querier.query(&QueryRequest::Bank(BankQuery::AllBalances {
        address
    }))
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {}
}