use bundlr_contracts_shared::{
    contract_utils::js_imports::{Block, Transaction},
    Address, BlockHeight,
};

use crate::{
    action::ActionResult, contract_utils::handler_result::HandlerResult, error::ContractError,
    state::State,
};

pub async fn leave(mut state: State) -> ActionResult {
    let caller = Transaction::owner()
        .parse::<Address>()
        .map_err(|err| ContractError::ParseError(err.to_string()))?;

    match state.bundlers.get_mut(&caller) {
        Some(val @ None) => {
            *val = Some(BlockHeight(
                Block::height() as u128 + state.withdraw_delay as u128,
            ))
        }
        Some(Some(block)) => {
            return Err(ContractError::AlreadyLeaving(caller, *block));
        }
        None => {
            return Err(ContractError::InvalidBundler(caller));
        }
    }

    Ok(HandlerResult::NewState(state))
}
