use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Uint128, Uint64, Coin};
use cw_utils::Expiration;

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Option<String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    CreateCampaign {
        /// goal is the token amount for releasing tokens.
        goal: Uint128,
        // expiration date of the campaign
        expiration: Option<Expiration>,
        // name of the campaign
        campaign_name: String,
        // unique id of the campaign
        campaign_id: u64
    },
    Fund { amount: Vec<Coin>, campaign_id: u64 },
    Withdraw { campaign_id: u64 },
    // CancelCampaign {},
}

pub enum QueryMsg {
    ViewCampaign { campaign_id: u64 },
}