use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use bundlr_contracts_shared::{Address, Amount, BlockHeight};

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub token: Address,
    pub bundlers: HashMap<Address, Option<BlockHeight>>,
    pub withdraw_delay: u16,
    pub stake: Amount,
    pub allowed_interactors: HashSet<Address>,
}
