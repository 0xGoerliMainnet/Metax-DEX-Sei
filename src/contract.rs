#[cfg(not(feature = "library"))]
use std::str;

use cosmwasm_std::{
    entry_point, to_binary, Addr, CosmosMsg, Decimal, Deps, DepsMut, Env,
    MessageInfo, Response, StdError, StdResult, Uint128, WasmMsg, WasmQuery, Coin
};

use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg, SwapOperation};
use crate::state::{State, STATE};

// sparrowswap
use sparrowswap_lib::pair::{ExecuteMsg as SparrowSwapeMsg};
use sparrowswap_lib::asset::{Asset as SparrowSwapAsset};

// astroport
use astroport_lib::pair::{ExecuteMsg as AstroportMsg};
use astroport_lib::asset::{Asset as AstroportAsset, AssetInfo};
use astroport_lib::querier::{query_balance, query_pair_info, query_token_balance};

use serde_json::{Result as JSONResult, Value};


// version info for migration info
const CONTRACT_NAME: &str = "crates.io:wasm-dexrouter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        count: msg.count,
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Increment {} => execute::increment(deps),
        ExecuteMsg::Reset { count } => execute::reset(deps, info, count),
        ExecuteMsg::SparrowSwap {
            pool_address,
            offer_asset,
            ask_asset_info,
            belief_price,
            max_spread,
            to
        } => execute::sparrowSwap( deps, env, info, pool_address, offer_asset, ask_asset_info, belief_price, max_spread, to),
        ExecuteMsg::AstroportSwap {
            pool_address,
            offer_asset,
            ask_asset_info,
            belief_price,
            max_spread,
            to
        } => execute::astroportSwap(deps, env, info, pool_address, offer_asset, ask_asset_info, belief_price, max_spread, to),
        ExecuteMsg::Unxswap{
            steps,
            minimum_receive,
            to
        } => execute::unxswap(deps, env, info, steps, minimum_receive, to),
        ExecuteMsg::AssertMinimumReceive {
            asset_info,
            prev_balance,
            minimum_receive,
            receiver,
        } => execute::assert_minimum_receive(
            deps.as_ref(),
            asset_info,
            prev_balance,
            minimum_receive,
            deps.api.addr_validate(&receiver)?,
        ),
        
    }

}

pub mod execute {
    use super::*;

    pub fn increment(deps: DepsMut) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            state.count += 1;
            Ok(state)
        })?;

        Ok(Response::new().add_attribute("action", "increment"))
    }

    pub fn reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            if info.sender != state.owner {
                return Err(ContractError::Unauthorized {});
            }
            state.count = count;
            Ok(state)
        })?;
        Ok(Response::new().add_attribute("action", "reset"))
    }

    pub fn sparrowSwap(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        pool_address: String,
        offer_asset: SparrowSwapAsset,
        ask_asset_info: Option<AssetInfo>,
        belief_price: Option<Decimal>,
        max_spread: Option<Decimal>,
        to: Option<String>,
    ) -> Result<Response, ContractError> {

        let mut temp_ask_asset_info = ask_asset_info.clone().unwrap();
        let mut temp_ask_asset_info2 = ask_asset_info.clone().unwrap();

        // smart query
        let ask_balance = match temp_ask_asset_info {
            AssetInfo::NativeToken { denom } => {
                query_balance(&deps.querier, env.contract.address, denom)?
            }
            AssetInfo::Token { contract_addr } => {
                query_token_balance(&deps.querier, contract_addr, env.contract.address)?
            }
        };
        let ask_denom = str::from_utf8(temp_ask_asset_info2.as_bytes());
        let new_funds = vec![Coin{
            denom: (*(ask_denom.unwrap())).to_string(),
            amount: ask_balance
        }];


        Ok(Response::new().add_message(WasmMsg::Execute {
            contract_addr: pool_address,
            funds: new_funds,
            msg: to_binary(&SparrowSwapeMsg::Swap {
                offer_asset: offer_asset,
                belief_price: belief_price,
                max_spread: max_spread,
                to: to
            })?,
        }))
    }

    
    pub fn astroportSwap(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        pool_address: String,
        offer_asset: AstroportAsset,
        ask_asset_info: Option<AssetInfo>,
        belief_price: Option<Decimal>,
        max_spread: Option<Decimal>,
        to: Option<String>
    ) -> Result<Response, ContractError> {

        let mut temp_ask_asset_info = ask_asset_info.clone().unwrap();
        let mut temp_ask_asset_info2 = ask_asset_info.clone().unwrap();

        // smart query
        let ask_balance = match temp_ask_asset_info {
            AssetInfo::NativeToken { denom } => {
                query_balance(&deps.querier, env.contract.address, denom)?
            }
            AssetInfo::Token { contract_addr } => {
                query_token_balance(&deps.querier, contract_addr, env.contract.address)?
            }
        };
        let ask_denom = str::from_utf8(temp_ask_asset_info2.as_bytes());
        let new_funds = vec![Coin{
            denom: (*(ask_denom.unwrap())).to_string(),
            amount: ask_balance
        }];

        Ok(Response::new().add_message(WasmMsg::Execute {
            contract_addr: pool_address,
            funds: new_funds,
            msg: to_binary(&AstroportMsg::Swap {
                offer_asset: offer_asset,
                ask_asset_info: ask_asset_info,
                belief_price: belief_price,
                max_spread: max_spread,
                to: to
            })?,
        }))
    }

    pub fn assert_minimum_receive(
        deps: Deps,
        asset_info: AssetInfo,
        prev_balance: Uint128,
        minimum_receive: Uint128,
        receiver: Addr,
    ) -> Result<Response, ContractError> {
        asset_info.check(deps.api)?;
        let receiver_balance = asset_info.query_pool(&deps.querier, receiver)?;
        let swap_amount = receiver_balance.checked_sub(prev_balance)?;
    
        if swap_amount < minimum_receive {
            return Err(ContractError::AssertionMinimumReceive {
                receive: minimum_receive,
                amount: swap_amount,
            });
        }
    
        Ok(Response::default())
    }

    // only one step
    pub fn unxswap(
        deps: DepsMut,
        env: Env,
        raw_info: MessageInfo,
        steps: Vec<SwapOperation>,
        minimum_receive: Option<Uint128>,
        to: Option<Addr>
    ) -> Result<Response, ContractError> {
        let operations_len = steps.len();
        let mut operation_index = 0;
        let target_asset_info = steps.last().unwrap().get_target_asset_info();
        let to = if let Some(to) = to {
            deps.api.addr_validate(to.as_str())?
        } else {
            raw_info.sender
        };

        let mut messages: Vec<CosmosMsg> = steps
            .into_iter()
            .map(|op| {
                operation_index += 1;
                match op {
                    SwapOperation::SparrowSwap  {
                        pool_address,
                        offer_asset,
                        ask_asset_info,
                        belief_price,
                        max_spread,
                        base_swap_info,
                    } => {
                        Ok(
                            CosmosMsg::Wasm(
                                WasmMsg::Execute {
                                    contract_addr: env.contract.address.to_string(),
                                    funds: if operation_index == 0 {
                                            vec![Coin{
                                                denom: raw_info.funds[0].denom.clone(),
                                                amount: raw_info.funds[0].amount
                                            }] 
                                        } else {
                                            vec![]
                                        },                                   
                                    msg: to_binary(&ExecuteMsg::SparrowSwap {
                                        pool_address: pool_address,
                                        offer_asset: offer_asset,
                                        ask_asset_info: ask_asset_info,
                                        belief_price: belief_price,
                                        max_spread: max_spread,
                                        to: if operation_index == operations_len {
                                            Some(to.to_string())
                                        } else {
                                            None
                                        },
                                    })?,
                                }
                            )
                        )
                    },
                    SwapOperation::AstroportSwap  {
                        pool_address,
                        offer_asset,
                        ask_asset_info,
                        belief_price,
                        max_spread,
                        base_swap_info
                    } => {
                        Ok(CosmosMsg::Wasm(WasmMsg::Execute {
                            contract_addr: env.contract.address.to_string(),
                            funds: if operation_index == 0 {
                                    vec![Coin{
                                        denom: raw_info.funds[0].denom.clone(),
                                        amount: raw_info.funds[0].amount
                                    }] 
                                } else {
                                    vec![]
                                },
                            msg: to_binary(&ExecuteMsg::AstroportSwap {
                                pool_address: pool_address,
                                offer_asset: offer_asset,
                                ask_asset_info: ask_asset_info,
                                belief_price: belief_price,
                                max_spread: max_spread,
                                to: if operation_index == operations_len {
                                    Some(to.to_string())
                                } else {
                                    None
                                },
                            })?,
                        }))
                    }
                } 
            })
            .collect::<StdResult<Vec<CosmosMsg>>>()?;

        // Execute minimum amount assertion
        if let Some(minimum_receive) = minimum_receive {
            let receiver_balance = target_asset_info.query_pool(&deps.querier, to.clone())?;
            messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: env.contract.address.to_string(),
                funds: vec![],
                msg: to_binary(&ExecuteMsg::AssertMinimumReceive {
                    asset_info: target_asset_info,
                    prev_balance: receiver_balance,
                    minimum_receive,
                    receiver: to.to_string(),
                })?,
            }));
        }

        Ok(Response::new().add_messages(messages))

    }
}
