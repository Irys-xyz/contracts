use std::{collections::HashMap, panic};


use serde::Deserialize;

use crate::state::{State, Inner, Bundler};

#[derive(Deserialize)]
pub struct AddBundlerInput {
    url: String,
    amount: u64
}

pub fn add_bundler(mut inner: Inner, input: AddBundlerInput, caller: String) -> State {
    let AddBundlerInput { url, amount } = input;

    let bundlers = &mut inner.bundlers;

    if bundlers.contains_key(&caller) {
        panic!("Caller already a bundler")
    };

    // Check there are enough tokens the contract can move
    let enough = true;

    if !enough {
        panic!("Not enough tokens approved")
    };

    // Move tokens
    
    let bundler = Bundler {
        url,
        amount,
    };

    bundlers.insert(caller, bundler);

    State::New(inner)
}