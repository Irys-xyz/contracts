use std::collections::HashMap;

use crate::contract_utils::handler_result::HandlerResult;
use bundlr_contracts_shared::{Address, Amount, BlockHeight};
use serde::{Deserialize, Serialize};

use crate::{error::ContractError, state::State};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", tag = "function")]
pub enum Action {
    Bundlers,
    WithdrawDelay,
    Stake,
    Token,
    Join,
    Leave,
    SyncSlashed,
    Withdraw,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum QueryResponseMsg {
    Token(Address),
    Stake(Amount),
    Bundlers(HashMap<Address, Option<BlockHeight>>),
    WithdrawDelay(u16),
}

pub type ActionResult = Result<HandlerResult<State, QueryResponseMsg>, ContractError>;
