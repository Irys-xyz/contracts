use crate::action::{ActionResult, QueryResponseMsg};
use crate::contract_utils::handler_result::HandlerResult::QueryResponse;
use crate::state::State;

pub fn bunders(state: State) -> ActionResult {
    Ok(QueryResponse(QueryResponseMsg::Bundlers(state.bundlers)))
}
pub fn withdraw_delay(state: State) -> ActionResult {
    Ok(QueryResponse(QueryResponseMsg::WithdrawDelay(
        state.withdraw_delay,
    )))
}
pub fn stake(state: State) -> ActionResult {
    Ok(QueryResponse(QueryResponseMsg::Stake(state.stake)))
}
pub fn token(state: State) -> ActionResult {
    Ok(QueryResponse(QueryResponseMsg::Token(state.token)))
}
pub fn allowed_interactors(state: State) -> ActionResult {
    Ok(QueryResponse(QueryResponseMsg::AllowedInteractors(
        state.allowed_interactors,
    )))
}
