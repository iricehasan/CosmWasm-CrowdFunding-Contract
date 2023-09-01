use cosmwasm_std::{StdError,  Uint128};
use thiserror::Error;
use cw_utils::Expiration;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Campaign does not exist ({campaign_id:?})")]
    CampaignDoesNotExist { campaign_id: u64},

    #[error("Campaign already exists ({campaign_id:?})")]
    CampaignAlreadyExists { campaign_id: u64},

    #[error("expired campaign(expiration {expiration:?})")]
    Expired { expiration: Expiration },

    #[error("goal is already met(goal {goal:?})")]
    GoalIsAlreadyMet{ goal: Uint128 },

    #[error("goal is not met yet(goal {goal:?})")]
    GoalIsNotMetYet{ goal: Uint128},

    #[error("unauthorized")]
    Unauthorized {},

}