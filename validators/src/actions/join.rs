use std::str::FromStr;

use bundlr_contracts_shared::{
    contract_utils::js_imports::{Contract, SmartWeave},
    Address, Amount,
};
use serde::{Deserialize, Serialize};
use url::Url;
use wasm_bindgen::JsValue;

use crate::{
    action::ActionResult,
    contract_utils::handler_result::HandlerResult,
    error::ContractError,
    state::{State, Validator},
};

use crate::contract_utils::js_imports::log;

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

pub async fn join(mut state: State, stake: Amount, url: Url) -> ActionResult {
    log(&format!(
        "[validatorJoin] state {:?} stake {:?} url {:?}",
        state, stake, url
    ));
    let caller = SmartWeave::caller()
        .parse::<Address>()
        .map_err(|err| ContractError::ParseError(err.to_string()))?;

    if stake < state.minimum_stake {
        log(&format!(
            "[validatorJoin] invalid stake: state {:?} stake {:?}",
            state, stake
        ));
        return Err(ContractError::InvalidStake);
    }

    if state.validators.contains_key(&caller) {
        return Err(ContractError::AlreadyJoined);
    }

    let result = SmartWeave::write(
        &state.token.to_string(),
        JsValue::from_serde(&Input {
            function: "transferFrom".to_string(),
            from: caller.clone(),
            to: Address::from_str(&Contract::id())
                .map_err(|err| ContractError::ParseError(err.to_string()))?,
            amount: stake,
        })
        .unwrap(),
    )
    .await;

    let result: Result = result.into_serde().unwrap();

    if result.result_type != "ok" {
        log(&format!(
            "[validatorJoin] transferFrom call failed from {:?} amount {:?}",
            caller.clone(),
            stake
        ));
        return Err(ContractError::TransferFailed);
    }

    state.validators.insert(
        caller.clone(),
        Validator {
            address: caller,
            stake,
            url,
        },
    );

    let new_state = HandlerResult::NewState(state);
    log(&format!("[validatorJoin] state {:?}", new_state));
    Ok(new_state)
}
