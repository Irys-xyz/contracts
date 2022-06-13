use crate::{
    actions::slashing::{Proposal, Vote},
    contract_utils::handler_result::HandlerResult,
    epoch::Epoch,
};
use bundlr_contracts_shared::{Address, Amount, TransactionId};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{error::ContractError, state::State};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", tag = "function")]
pub enum Action {
    Validators,
    NominatedValidators,
    MinimumStake,
    Token,
    Epoch,
    EpochDuration,
    Bundler,
    BundlersContract,
    Join { stake: Amount, url: Url },
    Leave,
    UpdateEpoch,
    ProposeSlash { proposal: Proposal },
    VoteSlash { tx: TransactionId, vote: Vote },
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum QueryResponseMsg {
    Bundler(Address),
    BundlersContract(Address),
    Epoch(Epoch),
    EpochDuration(u16),
    Token(Address),
    Stake(Amount),
    Validators(Vec<Address>),
}

pub type ActionResult = Result<HandlerResult<State, QueryResponseMsg>, ContractError>;
