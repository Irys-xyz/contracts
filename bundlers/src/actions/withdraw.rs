use bundlr_contracts_shared::{
    contract_utils::js_imports::{log, Block, Contract, SmartWeave, Transaction},
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

pub async fn withdraw(mut state: State) -> ActionResult {
    log(&format!("withdraw caller: {}", SmartWeave::caller()));
    log(&format!("withdraw contract ID: {}", Contract::id()));
    log(&format!("withdraw tx ID: {}", Transaction::id()));
    log(&format!("withdraw tx owner: {}", Transaction::owner()));
    log(&format!("withdraw tx target: {}", Transaction::target()));
    log(&format!("withdraw block height: {}", Block::height()));
    let caller = Transaction::owner()
        .parse::<Address>()
        .map_err(|err| ContractError::ParseError(err.to_string()))?;

    let withdraw_allowed_in_block = match state.bundlers.get(&caller) {
        Some(Some(block)) => block,
        Some(None) => {
            return Err(ContractError::WithdrawBlocked);
        }
        None => {
            return Err(ContractError::InvalidBundler(caller));
        }
    };

    if **withdraw_allowed_in_block > Block::height() as u128 {
        log(&format!(
            "withdraw blocked, withdraw allowed in block {}, current block height {}",
            **withdraw_allowed_in_block,
            Block::height()
        ));
        return Err(ContractError::WithdrawBlocked);
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
    if result.result_type == "error" {
        return Err(ContractError::TransferFailed);
    }

    state.bundlers.remove(&caller);

    Ok(HandlerResult::NewState(state))
}
