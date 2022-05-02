use std::str::FromStr;

use bundlr_contracts_shared::Address;

use crate::action::{Action, ActionResult};
use crate::actions::allowance::{allowance, approve};
use crate::actions::queries::{balance, decimals, name, symbol, total_supply};
use crate::actions::transfer::{transfer, transfer_from};
use crate::error::ContractError;
use crate::state::State;

pub async fn handle(current_state: State, action: Action) -> ActionResult {
    match action {
        Action::BalanceOf { target } => {
            if let Ok(owner) = Address::from_str(target.as_str()) {
                balance(current_state, &owner)
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
        Action::TransferFrom { from, to, amount } => {
            match (from.parse(), to.parse(), amount.parse()) {
                (Ok(from), Ok(to), Ok(amount)) => transfer_from(current_state, from, to, amount),
                (Err(_), _, _) => Err(ContractError::InvalidAddress(from)),
                (_, Err(_), _) => Err(ContractError::InvalidAddress(to)),
                (_, _, Err(err)) => Err(ContractError::ParseError(err.to_string())),
            }
        }
        Action::Approve { spender, amount } => match (spender.parse(), amount.parse()) {
            (Ok(spender), Ok(amount)) => approve(current_state, spender, amount),
            (Err(_), _) => Err(ContractError::InvalidAddress(spender)),
            (_, Err(err)) => Err(ContractError::ParseError(err.to_string())),
        },
        Action::Allowance { owner, spender } => match (owner.parse(), spender.parse()) {
            (Ok(target), Ok(spender)) => allowance(current_state, target, spender),
            (_, Err(_)) => Err(ContractError::InvalidAddress(owner)),
            (Err(_), _) => Err(ContractError::InvalidAddress(spender)),
        },
    }
}
