use core::panic;
use serde::Deserialize;

use crate::state::{State, Inner};

#[derive(Deserialize)]
pub struct TransferInput {
    amount: u64,
    target: String
}

pub fn transfer(mut state: Inner, input: TransferInput, caller: String) -> State {

    let TransferInput { amount, target } = input;

    if amount == 0 { panic!("Must transfer > 0 BNDLR") };

    let balances = &mut state.balances;

    let caller_balance = balances.get_mut(&caller).unwrap();

    // if *caller_balance < amount { panic!("Not enough balance to transfer"); };

    *caller_balance -= amount;

    if balances.contains_key(&target) {
        *balances.get_mut(&target).unwrap() += amount;
    } else {
        balances.insert(target, amount);
    };

    return State::New { state };
}
