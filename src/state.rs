use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Uint128};
use cw_storage_plus::{Item};
use cw_utils::Expiration;

#[cw_serde]
pub struct Config {
    pub admin: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");

#[cw_serde]
pub struct Campaign {
    pub owner: Addr,
    pub name: String, // campaign name
    pub collected: Uint128,
    pub expiration: Option<Expiration>,
    pub goal: Uint128,
    pub funders: Vec<Funder>,
 }
 
 #[cw_serde]
 pub struct Funder {
    pub funder_address: Addr,
    pub contribution: Uint128,
 }

pub const CAMPAIGN: Item<Campaign> = Item::new("campaign");
// pub const CAMPAIGN_DATA: Map<u64, CampaignData> = Map::new("campaigns");
// pub const CONTRIBUTIONS: Map<Addr, U128> = Map::new("contributions"); // you can query contributions with this maybe?
