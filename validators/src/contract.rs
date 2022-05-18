use bundlr_contracts_shared::contract_utils::js_imports::{Block, SmartWeave, Transaction};
use bundlr_contracts_shared::{Address, TransactionId};

use crate::action::{Action, ActionResult};
use crate::actions;
use crate::error::ContractError;
use crate::state::State;

pub async fn handle(current_state: State, action: Action) -> ActionResult {
    let caller = SmartWeave::caller()
        .parse::<Address>()
        .map_err(|err| ContractError::ParseError(err.to_string()))?;
    let tx_id = Transaction::id()
        .parse::<TransactionId>()
        .map_err(|err| ContractError::ParseError(err.to_string()))?;
    let block_height = Block::height() as u128;

    match action {
        Action::Validators => actions::queries::validators(current_state),
        Action::NominatedValidators => actions::queries::nominated_validators(current_state),
        Action::MinimumStake => actions::queries::minimum_stake(current_state),
        Action::Token => actions::queries::token(current_state),
        Action::Bundler => actions::queries::bundler(current_state),
        Action::BundlersContract => actions::queries::bundlers_contract(current_state),
        Action::Epoch => actions::queries::epoch(current_state),
        Action::EpochDuration => actions::queries::epoch_duration(current_state),
        Action::Join { stake } => actions::join(current_state, stake).await,
        Action::Leave => actions::leave(current_state).await,
        Action::UpdateEpoch => actions::update_epoch(current_state).await,
        Action::ProposeSlash { proposal } => {
            actions::slashing::propose(current_state, caller, tx_id, block_height, proposal).await
        }
        Action::VoteSlash { tx, vote } => {
            actions::slashing::vote(current_state, caller, block_height, tx, vote).await
        }
    }
}
