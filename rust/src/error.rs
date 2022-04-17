use serde::Serialize;

use crate::action::{Address, Amount};

#[derive(Serialize)]
pub enum ContractError {
    RuntimeError(String),
    ParseError(String),
    InvalidAddress(String),
    AmountMustBeHigherThanZero,
    CallerBalanceNotEnough(Amount),
    InvalidSpenderAllowance {
        owner: Address,
        spender: Address,
        amount: Amount,
    },
}
