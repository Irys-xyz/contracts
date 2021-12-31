use serde::Deserialize;

use crate::state::{State, Inner};

#[derive(Deserialize)]
pub struct BalanceInput {
    target: String
}

pub fn balance(state: Inner, input: BalanceInput) -> State {
    let BalanceInput { target } = input;
    let balance = state.balances.get(&target).unwrap_or(&0);
    State::Balance(*balance)
}