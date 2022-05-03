use std::collections::HashMap;

use crate::{contract_utils::handler_result::HandlerResult, epoch::Epoch};
use bundlr_contracts_shared::{Address, Amount};
use serde::{Deserialize, Serialize};

use crate::{error::ContractError, state::State};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", tag = "function")]
pub enum Action {
    Validators,
    NominatedValidators,
    Stake,
    Token,
    Epoch,
    EpochDuration,
    Bundler,
    BundlersContract,
    Join,
    Leave,
    UpdateEpoch,
    ProposeSlash,
    VoteSlash,
    SyncSlashed,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum QueryResponseMsg {
    Bundler(Address),
    BundlersContract(Address),
    Epoch(Epoch),
    EpochDuration(u16),
    Token(Address),
    Stake(Amount),
    Validators(HashMap<Address, bool>),
}

pub type ActionResult = Result<HandlerResult<State, QueryResponseMsg>, ContractError>;
