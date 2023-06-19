#[cfg(not(feature = "library"))]

use cosmwasm_std::{
    entry_point, from_binary, to_binary, Addr, Api, Binary, CosmosMsg, Decimal, Deps, DepsMut, Env,
    MessageInfo, QueryRequest, Response, StdError, StdResult, Uint128, WasmMsg, WasmQuery, Coin
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
            belief_price,
            max_spread,
            to
        } => execute::sparrowSwap(info, pool_address, offer_asset, belief_price, max_spread, to),
        ExecuteMsg::AstroportSwap {
            pool_address,
            offer_asset,
            ask_asset_info,
            belief_price,
            max_spread,
            to
        } => execute::astroportSwap(info, pool_address, offer_asset, ask_asset_info, belief_price, max_spread, to),
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
        info: MessageInfo,
        pool_address: String,
        offer_asset: SparrowSwapAsset,
        belief_price: Option<Decimal>,
        max_spread: Option<Decimal>,
        to: Option<String>
    ) -> Result<Response, ContractError> {
        Ok(Response::new().add_message(WasmMsg::Execute {
            contract_addr: pool_address,
            funds: info.funds,
            msg: to_binary(&SparrowSwapeMsg::Swap {
                offer_asset: offer_asset,
                belief_price: belief_price,
                max_spread: max_spread,
                to: to
            })?,
        }))
    }


    
    pub fn astroportSwap(
        info: MessageInfo,
        pool_address: String,
        offer_asset: AstroportAsset,
        ask_asset_info: Option<AssetInfo>,
        belief_price: Option<Decimal>,
        max_spread: Option<Decimal>,
        to: Option<String>
    ) -> Result<Response, ContractError> {
        Ok(Response::new().add_message(WasmMsg::Execute {
            contract_addr: pool_address,
            funds: info.funds,
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
                        belief_price,
                        max_spread,
                        base_swap_info,
                    } => {
                        Ok(
                            CosmosMsg::Wasm(
                                WasmMsg::Execute {
                                    contract_addr: pool_address,
                                    funds: vec![Coin{
                                        denom: raw_info.funds[0].denom.clone(),
                                        amount: raw_info.funds[0].amount
                                    }],
                                    msg: to_binary(&SparrowSwapeMsg::Swap {
                                        offer_asset: offer_asset,
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
                            contract_addr: pool_address,
                            funds: vec![Coin{
                                denom: raw_info.funds[0].denom.clone(),
                                amount: raw_info.funds[0].amount
                            }],
                            msg: to_binary(&AstroportMsg::Swap {
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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query::count(deps)?),
    }
}

pub mod query {
    use super::*;

    pub fn count(deps: Deps) -> StdResult<GetCountResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(GetCountResponse { count: state.count })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(17, value.count);
    }

    

    #[test]
    fn increment() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(18, value.count);
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(5, value.count);
    }
}
