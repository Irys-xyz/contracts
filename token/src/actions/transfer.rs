use crate::action::{ActionResult, Address, Amount};
use crate::contract_utils::handler_result::HandlerResult;
use crate::contract_utils::js_imports::{log, SmartWeave, Transaction};
use crate::error::ContractError;
use crate::state::State;

use super::allowance::spend_allowance;

pub fn transfer(mut state: State, to: Address, amount: Amount) -> ActionResult {
    log(("caller ".to_owned() + &SmartWeave::caller()).as_str());
    log(("Transaction owner ".to_owned() + &Transaction::owner()).as_str());

    if amount == Amount::ZERO {
        return Err(ContractError::AmountMustBeHigherThanZero);
    }

    // TODO: why `Transaction::owner()` and not `SmartWeave::caller()`?
    let caller = Transaction::owner()
        .parse::<Address>()
        .map_err(|err| ContractError::ParseError(err.to_string()))?;
    let balances = &mut state.balances;

    // Checking if caller has enough funds
    let caller_balance = *balances.get(&caller).unwrap_or(&Amount::ZERO);
    if caller_balance <= amount {
        return Err(ContractError::CallerBalanceNotEnough(caller_balance));
    }

    // Update caller balance
    balances.insert(caller, caller_balance - amount);

    // Update target balance
    let target_balance = *balances.get(&to).unwrap_or(&Amount::ZERO);
    balances.insert(to, target_balance + amount);

    Ok(HandlerResult::NewState(state))
}

pub fn transfer_from(state: State, from: Address, to: Address, amount: Amount) -> ActionResult {
    log(("caller ".to_owned() + &SmartWeave::caller()).as_str());
    log(("Transaction owner ".to_owned() + &Transaction::owner()).as_str());

    if amount == Amount::ZERO {
        return Err(ContractError::AmountMustBeHigherThanZero);
    }

    // TODO: why `Transaction::owner()` and not `SmartWeave::caller()`?
    let caller = Transaction::owner()
        .parse::<Address>()
        .map_err(|err| ContractError::ParseError(err.to_string()))?;

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
    if from_balance <= amount {
        return Err(ContractError::CallerBalanceNotEnough(from_balance));
    }

    // Update caller balance
    // TODO: implement SubAssign for Amount
    balances.insert(from, from_balance - amount);

    // Update target balance
    let to_balance = *balances.get(&to).unwrap_or(&Amount::ZERO);
    balances.insert(to, to_balance + amount);

    Ok(HandlerResult::NewState(state))
}
