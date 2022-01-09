use serde::Deserialize;

use crate::state::{Inner, State};

#[derive(Deserialize)]
pub struct TransferFromInput {

}

pub fn transfer_from(mut state: Inner, input: TransferFromInput, caller: String) -> State {


    State::New(state.clone())
}
