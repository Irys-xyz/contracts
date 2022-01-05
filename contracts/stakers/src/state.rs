use std::{collections::HashMap};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Inner {
    pub bundler_contract: String,
    // delegator address => delegation
    pub delegations: HashMap<String, Vec<Delegation>>,
    // bundler address => total
    pub delegation_totals: HashMap<String, u64>
}

#[derive(Serialize, Deserialize)]
pub struct Delegation {
    pub bundler: String,
    pub amount: u64
}

#[derive(Serialize, Deserialize)]
pub enum State {
    New(Inner),
    StakeBalance(u64)
}