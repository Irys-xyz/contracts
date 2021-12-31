use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct Inner {
    pub bundlers: HashMap<String, Bundler>,

    #[serde(rename = "foreignCalls")]
    foreign_calls: Vec<ForeignCall>
}

#[derive(Serialize, Deserialize)]
pub struct ForeignCall {
    function: String,
    #[serde(rename = "foreignContract")]
    foreign_contract: String,
    invocation: Value
}

#[derive(Serialize, Deserialize)]
pub enum State {
    New(Inner)
}

#[derive(Serialize, Deserialize)]
pub struct Bundler {
    pub url: String,
    pub amount: u64
}