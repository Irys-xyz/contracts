use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use bundlr_contracts_shared::{Address, Amount, TransactionId};
use url::Url;

use crate::{
    actions::slashing::{Proposal, Voting},
    epoch::Epoch,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Validator {
    pub address: Address,
    pub url: Url,
    pub stake: Amount,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub bundler: Address,
    pub bundlers_contract: Address,
    pub epoch: Epoch,
    pub epoch_duration: u16,
    pub minimum_stake: Amount,
    pub token: Address,
    pub max_num_nominated_validators: u8,
    pub validators: HashMap<Address, Validator>,
    pub nominated_validators: Vec<Address>,
    pub slash_proposal_lifetime: u16,

    // key: TransactionId, ID of the tx that proposal is referring to
    // value.0: Proposal, the actual proposal data (matching with validator's sign request data)
    // value.1: Address, address/caller who is made the proposal
    // value.2: u128, block height when the proposal was made
    // value.3: TransactionId, tx ID in which the proposal was made
    // value.4: Voting, voting data for this proposal
    pub slash_proposals: HashMap<TransactionId, (Proposal, Address, u128, TransactionId, Voting)>,
}
