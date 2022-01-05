use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Inner {
    owner: String,
    total_supply: u64,
    decimals: u64,
    pub balances: HashMap<String, u64>,
    // address => (contract => amount)
    pub allowances: HashMap<String, HashMap<String, u64>>
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum State {
    New { state: Inner },
    Balance(u64)
}