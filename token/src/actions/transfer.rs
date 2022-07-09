use bundlr_contracts_shared::{contract_utils::js_imports::SmartWeave, Address, Amount};

use crate::action::ActionResult;
use crate::contract_utils::handler_result::HandlerResult;
use crate::contract_utils::js_imports::log;
use crate::error::ContractError;
use crate::state::State;

use super::allowance::spend_allowance;

pub fn transfer(mut state: State, to: Address, amount: Amount) -> ActionResult {
    if amount == Amount::ZERO {
        return Err(ContractError::AmountMustBeHigherThanZero);
    }

    let caller = SmartWeave::caller()
        .parse::<Address>()
        .map_err(|err| ContractError::ParseError(err.to_string()))?;
    let balances = &mut state.balances;

    // Checking if caller has enough funds
    let caller_balance = *balances.get(&caller).unwrap_or(&Amount::ZERO);
    if caller_balance < amount {
        return Err(ContractError::InvalidBalance(caller_balance));
    }

    balances.insert(caller, caller_balance - amount);

    let target_balance = *balances.get(&to).unwrap_or(&Amount::ZERO);
    balances.insert(to, target_balance + amount);

    let new_state = HandlerResult::NewState(state);

    log(&format!("[transfer] state {:?}", new_state));

    Ok(new_state)
}

pub fn transfer_from(state: State, from: Address, to: Address, amount: Amount) -> ActionResult {
    if amount == Amount::ZERO {
        log("[transferFrom] Amount is zero");
        return Err(ContractError::AmountMustBeHigherThanZero);
    }

    let caller = SmartWeave::caller().parse::<Address>().map_err(|err| {
        log(&format!("[transferFrom] Failed to parse address - {}", err));
        ContractError::ParseError(err.to_string())
    })?;

    // If caller is using this function for transferring tokens,
    // no need to check or change allowances.
    let mut state = if caller != from {
        spend_allowance(state, &from, &caller, &amount)?
    } else {
        state
    };

    let balances = &mut state.balances;

    // Checking if caller has enough funds
    let from_balance = *balances.get(&from).unwrap_or(&Amount::ZERO);
    if from_balance < amount {
        log(&format!(
            "transferFrom] Not enough balance. Expected >={:?} Got {:?}",
            amount, from_balance
        ));
        return Err(ContractError::InvalidBalance(from_balance));
    }

    balances.insert(from, from_balance - amount);

    let to_balance = *balances.get(&to).unwrap_or(&Amount::ZERO);
    balances.insert(to, to_balance + amount);

    Ok(HandlerResult::NewState(state))
}
