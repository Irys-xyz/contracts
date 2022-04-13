use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::action::{Address, Amount};

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub ticker: String,
    pub name: Option<String>,
    pub decimals: u8,
    pub total_supply: Amount,
    pub owner: Address,
    pub balances: HashMap<Address, Amount>,
}
