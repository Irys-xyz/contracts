use std::collections::HashMap;

use serde::Deserialize;

use crate::state::{State, Inner};

#[derive(Deserialize)]
pub struct ApproveInput {
    amount: u64,
    target: String
}

pub fn approve(mut state: Inner, input: ApproveInput, caller: String) -> State {
    let ApproveInput { amount, target } = input;

    if amount == 0 { panic!("Must approve non-zero amount") };

    let balance = state.balances.get(&caller).expect("No balance");

    if *balance < amount { panic!("Not enough balance") };

    if let Some(caller_allowances) = state.allowances.get_mut(&caller) {
        caller_allowances.insert(target, amount);
    } else {
        let allowance = HashMap::with_capacity(1);
        state.allowances.insert(caller, allowance);
    }

    let inner = state.clone();

    State::New(inner)
}