use crate::action::{Action, ActionResult, Address, QueryResponseMsg};
use crate::actions::queries::{balance, decimals, name, symbol, total_supply};
use crate::error::ContractError;
use crate::state::State;

pub async fn handle(current_state: State, action: Action) -> ActionResult {
    match action {
        Action::BalanceOf { target } => {
            if let Ok(address) = Address::try_from(target.as_str()) {
                balance(current_state, &address)
            } else {
                Err(ContractError::InvalidAddress(target.to_string()))
            }
        }
        Action::Name => name(current_state),
        Action::Symbol => symbol(current_state),
        Action::Decimals => decimals(current_state),
        Action::TotalSupply => total_supply(current_state),
        Action::Allowance { target, spender } => todo!(),
        Action::Transfer { to, amount } => todo!(),
        Action::TransferFrom { from, to, amount } => todo!(),
        Action::Approve { spender, amount } => todo!(),
    }
}
