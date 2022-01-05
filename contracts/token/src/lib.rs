use std::{panic, slice::SliceIndex};

use actions::{transfer::{transfer, TransferInput}, balance::{balance, BalanceInput}};
use serde::Deserialize;
use serde_json::Value;
use state::State;
use three_em::handler;

mod state;
mod actions;

#[link(wasm_import_module = "3em")]
extern "C" {
    #[allow(dead_code)]
    fn smartweave_read_state(
        // `ptr` is the pointer to the base64 URL encoded sha256 txid.
        ptr: *const u8,
        ptr_len: usize,
        // Pointer to the 4 byte array to store the length of the state.
        result_len_ptr: *mut u8,
    ) -> *mut u8;
}


#[allow(dead_code)]
fn read_state(tx_id: &[u8]) -> Value {
    let mut len = [0u8; 4];
    let state_ptr = unsafe {
      smartweave_read_state(tx_id.as_ptr(), tx_id.len(), len.as_mut_ptr())
    };
  
    let len = u32::from_le_bytes(len) as usize;
    let state = unsafe { Vec::from_raw_parts(state_ptr, len, len) };
  
    serde_json::from_slice(&state).unwrap()
}


#[derive(Deserialize)]
pub struct Action {
    caller: String,
    input: Input
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", tag = "function")]
pub enum Input {
  Transfer {
    input: TransferInput,
  },
  Balance {
    input: BalanceInput,
  }
}

#[handler]
fn handler(state: State, action: Action) -> State {
    let Action { input, caller } = action;

    let inner = match state {
        State::New { state } => state,
        _ => panic!("Must provide correct state for transfer"),
    };

    match input {
        Input::Transfer { input } => {
            transfer(inner, input, caller)
        },
        Input::Balance { input } => {
            balance(inner, input)
        },
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
