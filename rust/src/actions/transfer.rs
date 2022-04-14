use crate::action::{ActionResult, Address, Amount};
use crate::contract_utils::handler_result::HandlerResult;
use crate::contract_utils::js_imports::{log, SmartWeave, Transaction};
use crate::error::ContractError;
use crate::state::State;

pub fn transfer(mut state: State, to: Address, amount: Amount) -> ActionResult {
    log(("caller ".to_owned() + &SmartWeave::caller()).as_str());
    log(("Transaction owner ".to_owned() + &Transaction::owner()).as_str());

    if amount == Amount::ZERO {
        return Err(ContractError::TransferAmountMustBeHigherThanZero);
    }

    let caller = Transaction::owner()
        .parse::<Address>()
        .map_err(|err| ContractError::ParseError(err.to_string()))?;
    let balances = &mut state.balances;

    // Checking if caller has enough funds
    let caller_balance = *balances.get(&caller).unwrap_or(&Amount::ZERO);
    if caller_balance < amount {
        return Err(ContractError::CallerBalanceNotEnough(caller_balance));
    }

    // Update caller balance
    balances.insert(caller, caller_balance - amount);

    // Update target balance
    let target_balance = *balances.get(&to).unwrap_or(&Amount::ZERO);
    balances.insert(to, target_balance + amount);

    Ok(HandlerResult::NewState(state))
}
