use serde::{Deserialize, Serialize};

use bundlr_contracts_shared::{Address, Amount};

use crate::epoch::Epoch;

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub bundler: Address,
    pub bundlers_contract: Address,
    pub epoch: Epoch,
    pub epoch_duration: u16,
    pub stake: Amount,
    pub token: Address,
    pub validators: Vec<Address>,
    pub nominated_validators: Vec<Address>,
}
