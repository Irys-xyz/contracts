use std::str::FromStr;

use crate::action::{Action, ActionResult, Address};
use crate::actions::queries::{balance, decimals, name, symbol, total_supply};
use crate::actions::transfer::transfer;
use crate::error::ContractError;
use crate::state::State;

pub async fn handle(current_state: State, action: Action) -> ActionResult {
    match action {
        Action::BalanceOf { target } => {
            if let Ok(address) = Address::from_str(target.as_str()) {
                balance(current_state, &address)
            } else {
                Err(ContractError::InvalidAddress(target))
            }
        }
        Action::Name => name(current_state),
        Action::Symbol => symbol(current_state),
        Action::Decimals => decimals(current_state),
        Action::TotalSupply => total_supply(current_state),
        Action::Transfer { to, amount } => match (to.parse(), amount.parse()) {
            (Ok(to), Ok(amount)) => transfer(current_state, to, amount),
            (Err(_), _) => Err(ContractError::InvalidAddress(to)),
            (_, Err(err)) => Err(ContractError::ParseError(err.to_string())),
        },
        Action::TransferFrom {
            from: _,
            to: _,
            amount: _,
        } => todo!(),
        Action::Approve {
            spender: _,
            amount: _,
        } => todo!(),
        Action::Allowance {
            target: _,
            spender: _,
        } => todo!(),
    }
}
