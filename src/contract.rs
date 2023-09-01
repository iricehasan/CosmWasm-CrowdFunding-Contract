use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr, Uint128, BankMsg, Coin
};
use cw_utils::Expiration;

use crate::state::{Config, CONFIG, Campaign, CAMPAIGN, Funder};
use crate::error::ContractError;
use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // initialize the admin

    let admin = msg 
    .admin
    .and_then(|addr_string| deps.api.addr_validate(addr_string.as_str()).ok())
    .unwrap_or(info.sender);

    let config = Config {
        admin: admin.clone(),
    };

    CONFIG.save(deps.storage, &config)?; // save the state

    // add response attribute
    Ok(Response::new()
    .add_attribute("method", "instantiate")
    .add_attribute("admin", admin))

}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    
    match msg {
        ExecuteMsg::Fund { amount, campaign_id } => fund(deps, info, env, amount, campaign_id),
        ExecuteMsg::CreateCampaign { goal, expiration, campaign_name, campaign_id } => create_campaign(deps, info, goal, expiration, campaign_name, campaign_id),
        ExecuteMsg::Withdraw { campaign_id } => withdraw(deps, info, campaign_id),
    }
}

// functions for execute

pub fn create_campaign(deps: DepsMut, info: MessageInfo, goal: Uint128, expiration: Option<Expiration>, campaign_name: String, campaign_id: u64) -> Result<Response, ContractError> {

    // check if campaign exists
    if (CAMPAIGN.may_load(deps.storage, campaign_id)?).is_some() {
        // campaign already exists
        return Err(ContractError::CampaignAlreadyExists { campaign_id: campaign_id })
    }

    // create campaign
    let campaign = Campaign {
        campaign_id,
        owner: info.sender.clone(),
        campaign_name,
        expiration,
        goal,
        collected: Uint128::zero(),
        funders: Vec::new(),
    };

    CAMPAIGN.save(deps.storage, campaign_id, &campaign)?;

    Ok(Response::new().add_attribute("action", "campaign created")
                    .add_attribute("campaign owner", campaign.owner))
}

pub fn fund(deps: DepsMut, info: MessageInfo, env: Env, amount: Vec<Coin>, campaign_id: u64) -> Result<Response, ContractError> {

    // TO DO

    // add the info.sender address to the funders array or something similar to that

    // Also, need a mapping to keep track of which funder address has contributed to what amount

    // check the sended funds are bigger than a minimum amount and this condition holds

    if !(CAMPAIGN.may_load(deps.storage, campaign_id)?.is_some()) {
        return Err(ContractError::CampaignDoesNotExist { campaign_id: campaign_id }); 
    }

        // load the campaign
    let mut campaign = CAMPAIGN.load(deps.storage, campaign_id)?;
       

    let funded = amount.iter().map(|coin| coin.amount).sum();
    campaign.collected += funded;


   // Check if the funder is already present
   if !campaign.funders.iter().any(|funder| &funder.funder_address == &info.sender) {
        // Add the new funder to the funders vector
        campaign.funders.push(Funder {
            funder_address: info.sender.clone(),
            contribution: funded,
        });
    }

    CAMPAIGN.save(deps.storage,campaign_id, &mut campaign)?; // save the state of campaign

    // check two things

    // if the goal is already met

    if campaign.collected >= campaign.goal {
        return Err(ContractError::GoalIsAlreadyMet { goal: campaign.goal });
    }

    // check if the campaing is expired
    if let Some(expiration) = campaign.expiration {
        if expiration.is_expired(&env.block) {
            return Err(ContractError::Expired { expiration });
        }
    }

    Ok(send(campaign.owner, amount, "fund"))
}

// this is a helper
fn send(to_address: Addr, amount: Vec<Coin>, action: &str) -> Response {
    Response::new()
        .add_message(BankMsg::Send {
            to_address: to_address.clone().into(),
            amount,
        })
        .add_attribute("action", action)
        .add_attribute("to", to_address)
}

pub fn withdraw(deps: DepsMut, info: MessageInfo, campaign_id: u64) -> Result<Response, ContractError>  {
    
    // check if the campaign exists

    if !(CAMPAIGN.may_load(deps.storage, campaign_id)?.is_some()) {
        return Err(ContractError::CampaignDoesNotExist { campaign_id: campaign_id })
    }


    let campaign = CAMPAIGN.load(deps.storage, campaign_id)?;
    let owner = campaign.owner;

        
    // check the owner so that only the owner can withdraw
    if info.sender != owner {
        return Err(ContractError::Unauthorized {});
    }

    if campaign.collected < campaign.goal{
        return Err(ContractError::GoalIsNotMetYet { goal: campaign.goal }) ;
    }

    // this should release the balance 
    let mut res = Response::new();
    res = res.add_message(BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![Coin {
            denom: "uatom".to_string(),
            amount: campaign.collected,
        }]
    });

    CAMPAIGN.remove(deps.storage, campaign_id); // delete the campaing

    res = res.add_attribute("action", "withdraw")
            .add_attribute("to", info.sender.to_string());

    Ok(res)
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ViewCampaign { campaign_id } => view_campaign(deps, campaign_id),
    }
}

fn view_campaign(deps: Deps, campaign_id: u64) -> StdResult<Binary> {
    let campaign = CAMPAIGN.load(deps.storage, campaign_id)?;

    to_binary(&campaign)
}