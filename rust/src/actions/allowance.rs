use crate::action::{ActionResult, Address, Amount, QueryResponseMsg};
use crate::contract_utils::handler_result::HandlerResult;
use crate::contract_utils::js_imports::{log, SmartWeave, Transaction};
use crate::error::ContractError;
use crate::state::State;

pub fn approve(mut state: State, spender: Address, amount: Amount) -> ActionResult {
    log(("caller ".to_owned() + &SmartWeave::caller()).as_str());
    log(("Transaction owner ".to_owned() + &Transaction::owner()).as_str());

    if amount == Amount::ZERO {
        return Err(ContractError::AmountMustBeHigherThanZero);
    }

    // TODO: why `Transaction::owner()` and not `SmartWeave::caller()`?
    let caller = Transaction::owner()
        .parse::<Address>()
        .map_err(|err| ContractError::ParseError(err.to_string()))?;

    // Checking if caller has enough funds
    let caller_balance = *state.balances.get(&caller).unwrap_or(&Amount::ZERO);
    if caller_balance <= amount {
        return Err(ContractError::CallerBalanceNotEnough(caller_balance));
    }

    // set allowances[caller][spender] = amount,
    // insert default when hashmap is missing value for a key
    *state
        .allowances
        .entry(caller)
        .or_default()
        .entry(spender)
        .or_default() = amount;

    Ok(HandlerResult::NewState(state))
}

pub fn allowance(state: State, owner: Address, spender: Address) -> ActionResult {
    let allowance = *state
        .allowances
        .get(&owner)
        .map_or(&Amount::ZERO, |spenders| {
            spenders.get(&spender).unwrap_or(&Amount::ZERO)
        });

    Ok(HandlerResult::QueryResponse(QueryResponseMsg::Allowance {
        allowance,
        ticker: state.ticker,
        owner,
        spender,
    }))
}

pub(super) fn spend_allowance(
    mut state: State,
    owner: &Address,
    spender: &Address,
    amount: &Amount,
) -> Result<State, ContractError> {
    log(("caller ".to_owned() + &SmartWeave::caller()).as_str());
    log(("Transaction owner ".to_owned() + &Transaction::owner()).as_str());

    if amount == &Amount::ZERO {
        return Err(ContractError::AmountMustBeHigherThanZero);
    }

    // set allowances[caller][spender] = amount,
    // insert default when hashmap is missing value for a key
    let allowance = state
        .allowances
        .get_mut(&owner)
        .ok_or_else(|| ContractError::InvalidSpenderAllowance {
            owner: owner.clone(),
            spender: spender.clone(),
            amount: amount.clone(),
        })?
        .get_mut(&spender)
        .ok_or_else(|| ContractError::InvalidSpenderAllowance {
            owner: owner.clone(),
            spender: spender.clone(),
            amount: amount.clone(),
        })?;

    if *allowance < *amount {
        Err(ContractError::InvalidSpenderAllowance {
            owner: owner.clone(),
            spender: spender.clone(),
            amount: amount.clone(),
        })
    } else {
        *allowance = *allowance - *amount;
        Ok(state)
    }
}
