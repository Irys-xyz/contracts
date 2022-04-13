use serde::Serialize;

use crate::action::Amount;

#[derive(Serialize)]
pub enum ContractError {
    RuntimeError(String),
    ParseError(String),
    InvalidAddress(String),
    TransferAmountMustBeHigherThanZero,
    CallerBalanceNotEnough(Amount),
}
