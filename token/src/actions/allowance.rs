use std::collections::hash_map::Entry;

use bundlr_contracts_shared::{
    contract_utils::js_imports::{log, SmartWeave, Transaction},
    Address, Amount,
};

use crate::action::{ActionResult, QueryResponseMsg};
use crate::contract_utils::handler_result::HandlerResult;
use crate::error::ContractError;
use crate::state::State;

pub fn approve(mut state: State, spender: Address, amount: Amount) -> ActionResult {
    log(&format!("token approve caller {}", SmartWeave::caller()));
    log(&format!(
        "token approve Transaction owner {}",
        Transaction::owner()
    ));

    let caller = SmartWeave::caller()
        .parse::<Address>()
        .map_err(|err| ContractError::ParseError(err.to_string()))?;

    // Checking if caller has enough funds
    let caller_balance = *state.balances.get(&caller).unwrap_or(&Amount::ZERO);
    if caller_balance <= amount {
        return Err(ContractError::CallerBalanceNotEnough(caller_balance));
    }

    match amount {
        Amount::ZERO => {
            if let Entry::Occupied(mut caller_allowances) = state.allowances.entry(caller) {
                if let Entry::Occupied(spender_allowance) =
                    caller_allowances.get_mut().entry(spender)
                {
                    spender_allowance.remove_entry();
                } else {
                    // nothing to do here, allowance for the spender is not set
                }

                // Check if we end up removing the last spender entry and remove the
                // entry matching for the caller to cleanup the state.
                if caller_allowances.get().keys().len() == 0 {
                    caller_allowances.remove_entry();
                }
            } else {
                // nothing to do here, caller has no allowances, so nothing to remove
            }
        }
        _ => {
            // set allowances[caller][spender] = amount,
            // insert default when hashmap is missing value for a key
            *state
                .allowances
                .entry(caller)
                .or_default()
                .entry(spender)
                .or_default() = amount;
        }
    }

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
    log(&format!(
        "token spend allowance caller {}",
        SmartWeave::caller()
    ));
    log(&format!(
        "token spend allowance Transaction owner {}",
        Transaction::owner()
    ));

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

        // If the remaining allowances is zero, cleanup
        if *allowance == Amount::ZERO {
            // We can safely unwrap here
            let owner_allowances = state.allowances.get_mut(&owner).unwrap();
            owner_allowances.remove(&spender).unwrap();
            if owner_allowances.keys().len() == 0 {
                state.allowances.remove(&owner);
            }
        }
        Ok(state)
    }
}
