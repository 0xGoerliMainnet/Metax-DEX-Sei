use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Decimal, Uint128, Addr};

// Sparrow Swap
use sparrowswap_lib::asset::{Asset as SparrowSwapAsset, };

// Astroport Swap
use astroport_lib::asset::{Asset as AstroportAsset, AssetInfo};

#[cw_serde]
pub struct InstantiateMsg {
    pub count: i32,
}

/// This enum describes a swap operation.
#[cw_serde]
pub enum SwapOperation {
    SparrowSwap  {
        pool_address: String,
        offer_asset: SparrowSwapAsset,
        belief_price: Option<Decimal>,
        max_spread: Option<Decimal>
    },
    AstroportSwap {
        pool_address: String,
        offer_asset: AstroportAsset,
        ask_asset_info: Option<AssetInfo>,
        belief_price: Option<Decimal>,
        max_spread: Option<Decimal>,
    }
}


#[cw_serde]
pub enum ExecuteMsg {
    Increment {},
    Reset { count: i32 },

    SparrowSwap  {
        pool_address: String,
        offer_asset: SparrowSwapAsset,
        belief_price: Option<Decimal>,
        max_spread: Option<Decimal>,
        to: Option<String>,
    },
    AstroportSwap {
        pool_address: String,
        offer_asset: AstroportAsset,
        ask_asset_info: Option<AssetInfo>,
        belief_price: Option<Decimal>,
        max_spread: Option<Decimal>,
        to: Option<String>,
    },
    Unxswap {
        steps: Vec<SwapOperation>,
        minimum_receive: Option<Uint128>,
        to: Option<Addr>,
        target_asset_info: AssetInfo,
    },
    AssertMinimumReceive {
        asset_info: AssetInfo,
        prev_balance: Uint128,
        minimum_receive: Uint128,
        receiver: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetCountResponse)]
    GetCount {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
    pub count: i32,
}
