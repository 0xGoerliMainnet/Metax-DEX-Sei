use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Decimal};

// Sparrow Swap
use sparrowswap_lib::asset::{Asset as SparrowSwapAsset};

// Astroport Swap
use astroport_lib::asset::{Asset as AstroportAsset, AssetInfo as AstroportAssetInfo};


#[cw_serde]
pub struct InstantiateMsg {
    pub count: i32,
}

#[cw_serde]
pub enum ExecuteMsg {
    Increment {},
    Reset { count: i32 },
    SparrowSwap {
        pool_address: String,
        offer_asset: SparrowSwapAsset,
        belief_price: Option<Decimal>,
        max_spread: Option<Decimal>,
        to: Option<String>,
    },
    AstroportSwap {
        pool_address: String,
        offer_asset: AstroportAsset,
        ask_asset_info: Option<AstroportAssetInfo>,
        belief_price: Option<Decimal>,
        max_spread: Option<Decimal>,
        to: Option<String>,
    }
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
