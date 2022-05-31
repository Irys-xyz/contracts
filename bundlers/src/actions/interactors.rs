use bundlr_contracts_shared::{contract_utils::js_imports::SmartWeave, Address};

use crate::{
    action::ActionResult, contract_utils::handler_result::HandlerResult, error::ContractError,
    state::State,
};

pub async fn add(mut state: State, owner: Address, interactor: Address) -> ActionResult {
    let caller = SmartWeave::caller()
        .parse::<Address>()
        .map_err(|err| ContractError::ParseError(err.to_string()))?;

    if caller != owner && !state.allowed_interactors.contains(&caller) {
        return Err(ContractError::Forbidden);
    }

    if state.allowed_interactors.contains(&interactor) {
        return Err(ContractError::AlreadyInteractor(interactor));
    }

    state.allowed_interactors.insert(interactor);

    Ok(HandlerResult::NewState(state))
}

pub async fn remove(mut state: State, owner: Address, interactor: Address) -> ActionResult {
    let caller = SmartWeave::caller()
        .parse::<Address>()
        .map_err(|err| ContractError::ParseError(err.to_string()))?;

    if caller != owner && !state.allowed_interactors.contains(&caller) {
        return Err(ContractError::Forbidden);
    }

    if !state.allowed_interactors.remove(&interactor) {
        return Err(ContractError::InvalidInteractor(interactor));
    }

    Ok(HandlerResult::NewState(state))
}
