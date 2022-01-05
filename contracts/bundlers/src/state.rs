use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Inner {
    pub bundlers: HashMap<String, Bundler>,
    foreign_calls: Vec<ForeignCall>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForeignCall {
    function: String,
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