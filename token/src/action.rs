use bundlr_contracts_shared::{Address, Amount};
use serde::{Deserialize, Serialize};

use crate::contract_utils::handler_result::HandlerResult;
use crate::error::ContractError;
use crate::state::State;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", tag = "function")]
pub enum Action {
    Name,
    Symbol,
    Decimals,
    TotalSupply,
    BalanceOf {
        target: String,
    },
    Allowance {
        owner: String,
        spender: String,
    },
    Approve {
        spender: String,
        amount: String,
    },
    Burn {
        amount: String,
    },
    BurnFrom {
        from: String,
        amount: String,
    },
    Transfer {
        to: String,
        amount: String,
    },
    TransferFrom {
        from: String,
        to: String,
        amount: String,
    },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum QueryResponseMsg {
    Allowance {
        allowance: Amount,
        ticker: String,
        owner: Address,
        spender: Address,
    },
    Balance {
        balance: Amount,
        ticker: String,
        target: Address,
    },
    Decimals(u8),
    Name(Option<String>),
    Symbol(String),
    TotalSupply(Amount),
}

pub type ActionResult = Result<HandlerResult<State, QueryResponseMsg>, ContractError>;
