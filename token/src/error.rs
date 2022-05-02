use bundlr_contracts_shared::{Address, Amount};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
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
