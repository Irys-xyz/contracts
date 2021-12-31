use std::{collections::HashMap};

use serde::Deserialize;
use serde_json::Value;

use crate::{state::{Inner, Delegation, State}, read_state};

#[derive(Deserialize)]
pub struct DelegateInput {
    bundler: String,
    amount: u64
}

#[derive(Deserialize)]
pub struct BundlerState {
    bundlers: HashMap<String, Value>
}

pub fn delegate(mut inner: Inner, input: DelegateInput, caller: String) -> State {
    let DelegateInput { bundler, amount } = input;

    let token_contract_state = read_state::<BundlerState>(inner.bundler_contract.as_bytes());
    let bundler_contract_state = read_state::<BundlerState>(inner.bundler_contract.as_bytes());

    if !bundler_contract_state.bundlers.contains_key(&bundler) { panic!("Must delegate to a valid bundler") };

    

    let delegations = &mut inner.delegations;
    let totals = &mut inner.delegation_totals;

    if let Some(d) = delegations.get_mut(&caller) {
        for delegation in d.iter_mut() {
            if delegation.bundler == bundler {
                delegation.amount += amount;
                return State::New(inner);
            }
        }

        let delegation = Delegation {
            bundler,
            amount
        };

        d.push(delegation);
        return State::New(inner);
    };

    if let Some(t) = totals.get_mut(&bundler) {
        *t += amount;
    } else {
        totals.insert(bundler.clone(), amount);
    }

    let delegation = Delegation {
        bundler,
        amount
    };

    delegations.insert(caller, vec![delegation]);

    State::New(inner)
}