use bundlr_contracts_shared::{
    contract_utils::js_imports::{SmartWeave, Transaction},
    Address, Amount,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

use crate::{
    action::ActionResult, contract_utils::handler_result::HandlerResult, error::ContractError,
    state::State,
};

#[derive(Serialize)]
struct Input {
    function: String,
    to: Address,
    amount: Amount,
}

#[derive(Debug, Deserialize)]
struct Result {
    #[serde(rename = "type")]
    result_type: String,
}

pub async fn leave(mut state: State) -> ActionResult {
    let caller = Transaction::owner()
        .parse::<Address>()
        .map_err(|err| ContractError::ParseError(err.to_string()))?;

    if state.nominated_validators.contains(&caller) {
        return Err(ContractError::NominatedValidatorCannotLeave(caller));
    }

    if let Some(index) = state
        .validators
        .iter()
        .position(|address| address == &caller)
    {
        state.validators.remove(index);
    } else {
        return Err(ContractError::InvalidValidator(caller));
    }

    let result = SmartWeave::write(
        &state.token.to_string(),
        JsValue::from_serde(&Input {
            function: "transfer".to_string(),
            to: caller.clone(),
            amount: state.stake,
        })
        .unwrap(),
    )
    .await;

    let result: Result = result.into_serde().unwrap();

    // TODO: is this reliable enough as a check?
    if result.result_type != "ok" {
        return Err(ContractError::TransferFailed);
    }

    Ok(HandlerResult::NewState(state))
}
