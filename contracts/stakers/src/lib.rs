mod state;
mod actions;

use actions::delegate::{DelegateInput, delegate};
use serde::{Deserialize, de::DeserializeOwned};
use serde_json::Value;
use state::State;
use three_em::handler;


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

fn read_state<T: DeserializeOwned>(tx_id: &[u8]) -> T {
    let mut len = [0u8; 4];
    let state_ptr = unsafe {
      smartweave_read_state(tx_id.as_ptr(), tx_id.len(), len.as_mut_ptr())
    };
  
    let len = u32::from_le_bytes(len) as usize;
    let state = unsafe { Vec::from_raw_parts(state_ptr, len, len) };
  
    serde_json::from_slice::<T>(&state).unwrap()
}



#[derive(Deserialize)]
pub struct Action<T> {
    function: Method,
    caller: String,
    #[serde(bound(deserialize = "T: Deserialize<'de>"))]
    input: T
}

#[derive(Deserialize)]
pub enum Method {
    #[serde(rename = "delegate")]
    Delegate,

    #[serde(rename = "undelegate")]
    Undelegate
}

fn handler(state: State, action: Action<Value>) -> State {
    let Action { function, input, caller } = action;

    match function {
        Method::Delegate => {
            if let State::New(inner) = state {
                let input = serde_json::from_value::<DelegateInput>(input).unwrap();
                delegate(inner, input, caller)
            } else {
                panic!("Must provide correct state for transfer")
            }
        }
        Method::Undelegate => todo!(),
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
