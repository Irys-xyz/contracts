use bundlr_contracts_shared::{Address, Amount};

use crate::action::{ActionResult, QueryResponseMsg};
use crate::contract_utils::handler_result::HandlerResult::QueryResponse;
use crate::state::State;

pub fn name(state: State) -> ActionResult {
    Ok(QueryResponse(QueryResponseMsg::Name(state.name)))
}

pub fn symbol(state: State) -> ActionResult {
    Ok(QueryResponse(QueryResponseMsg::Symbol(state.ticker)))
}

pub fn decimals(state: State) -> ActionResult {
    Ok(QueryResponse(QueryResponseMsg::Decimals(state.decimals)))
}

pub fn total_supply(state: State) -> ActionResult {
    Ok(QueryResponse(QueryResponseMsg::TotalSupply(
        state.total_supply,
    )))
}

pub fn balance(state: State, target: &Address) -> ActionResult {
    let balance = state.balances.get(target).unwrap_or(&Amount::ZERO);

    Ok(QueryResponse(QueryResponseMsg::Balance {
        balance: *balance,
        ticker: state.ticker,
        target: target.clone(),
    }))
}
