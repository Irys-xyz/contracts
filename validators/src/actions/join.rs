use std::str::FromStr;

use bundlr_contracts_shared::{
    contract_utils::js_imports::{Contract, SmartWeave, Transaction},
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
    from: Address,
    to: Address,
    amount: Amount,
}

#[derive(Debug, Deserialize)]
struct Result {
    #[serde(rename = "type")]
    result_type: String,
}

pub async fn join(mut state: State) -> ActionResult {
    // TODO: should this be SmartWeave::caller instead?
    let caller = Transaction::owner()
        .parse::<Address>()
        .map_err(|err| ContractError::ParseError(err.to_string()))?;

    let result = SmartWeave::write(
        &state.token.to_string(),
        JsValue::from_serde(&Input {
            function: "transferFrom".to_string(),
            from: caller.clone(),
            to: Address::from_str(&Contract::id())
                .map_err(|err| ContractError::ParseError(err.to_string()))?,
            amount: state.stake,
        })
        .unwrap(),
    )
    .await;

    let result: Result = result.into_serde().unwrap();

    // TODO: is this reliable enough as a check?
    if result.result_type == "error" {
        return Err(ContractError::TransferFailed);
    }

    state.validators.push(caller);

    Ok(HandlerResult::NewState(state))
}
