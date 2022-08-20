use bundlr_contracts_shared::{Address, Amount};

use serde::{Deserialize, Serialize};

#[cfg(feature = "js-runtime")]
use bundlr_contracts_shared::contract_utils::js_imports::SmartWeave;

#[cfg(feature = "js-runtime")]
use wasm_bindgen::JsValue;

#[cfg(feature = "js-runtime")]
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

#[cfg(feature = "js-runtime")]
pub async fn leave(mut state: State) -> ActionResult {
    let caller = SmartWeave::caller()
        .parse::<Address>()
        .map_err(|err| ContractError::ParseError(err.to_string()))?;

    if state.nominated_validators.contains(&caller) {
        return Err(ContractError::NominatedValidatorCannotLeave(caller));
    }

    // FIXME: if the caller has voted in a currently open slash proposal, prevent leaving

    let validator = if let Some(validator) = state.validators.remove(&caller) {
        validator
    } else {
        return Err(ContractError::InvalidValidator(caller));
    };

    let result = SmartWeave::write(
        &state.token.to_string(),
        JsValue::from_serde(&Input {
            function: "transfer".to_string(),
            to: caller.clone(),
            amount: validator.stake,
        })
        .unwrap(),
    )
    .await;

    let result: Result = result.into_serde().unwrap();

    if result.result_type != "ok" {
        return Err(ContractError::TransferFailed);
    }

    Ok(HandlerResult::NewState(state))
}
