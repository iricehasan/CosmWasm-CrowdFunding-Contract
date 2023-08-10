use cosmwasm_std::{StdError,  Uint128};
use thiserror::Error;
use cw_utils::Expiration;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("expired campaign(expiration {expiration:?})")]
    Expired { expiration: Expiration },

    #[error("goal is already met(goal {goal:?})")]
    GoalIsAlreadyMet{ goal: Uint128 },

    #[error("goal is not met yet(goal {goal:?})")]
    GoalIsNotMetYet{ goal: Uint128},

    #[error("unauthorized")]
    Unauthorized {},

}