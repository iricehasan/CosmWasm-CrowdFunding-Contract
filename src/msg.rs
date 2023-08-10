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
        name: String,
    },
    Fund { amount: Vec<Coin> },
    Withdraw {},
    // CancelCampaign {},
}

pub enum QueryMsg {
    Unimplemented()
}