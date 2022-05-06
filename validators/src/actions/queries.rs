use crate::action::{ActionResult, QueryResponseMsg};
use crate::contract_utils::handler_result::HandlerResult::QueryResponse;
use crate::state::State;

pub fn validators(state: State) -> ActionResult {
    Ok(QueryResponse(QueryResponseMsg::Validators(
        state.validators,
    )))
}

pub fn nominated_validators(state: State) -> ActionResult {
    Ok(QueryResponse(QueryResponseMsg::Validators(
        state.nominated_validators,
    )))
}

pub fn stake(state: State) -> ActionResult {
    Ok(QueryResponse(QueryResponseMsg::Stake(state.stake)))
}

pub fn token(state: State) -> ActionResult {
    Ok(QueryResponse(QueryResponseMsg::Token(state.token)))
}

pub fn epoch(state: State) -> ActionResult {
    Ok(QueryResponse(QueryResponseMsg::Epoch(state.epoch)))
}

pub fn epoch_duration(state: State) -> ActionResult {
    Ok(QueryResponse(QueryResponseMsg::EpochDuration(
        state.epoch_duration,
    )))
}

pub fn bundler(state: State) -> ActionResult {
    Ok(QueryResponse(QueryResponseMsg::Bundler(state.bundler)))
}

pub fn bundlers_contract(state: State) -> ActionResult {
    Ok(QueryResponse(QueryResponseMsg::BundlersContract(
        state.bundlers_contract,
    )))
}
