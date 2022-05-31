use bundlr_contracts_shared::contract_utils::js_imports::Contract;
use bundlr_contracts_shared::Address;

use crate::action::{Action, ActionResult};
use crate::actions;
use crate::error::ContractError;
use crate::state::State;

pub async fn handle(current_state: State, action: Action) -> ActionResult {
    match action {
        Action::Bundlers => actions::queries::bunders(current_state),
        Action::WithdrawDelay => actions::queries::withdraw_delay(current_state),
        Action::Stake => actions::queries::stake(current_state),
        Action::Token => actions::queries::token(current_state),
        Action::Join => actions::join(current_state).await,
        Action::Leave => actions::leave(current_state).await,
        Action::Withdraw => actions::withdraw(current_state).await,
        Action::SyncSlashed => {
            // Delete bundler and move all tokens to a treasury address.
            todo!()
        }
        Action::AllowedInteractors => actions::queries::allowed_interactors(current_state),
        Action::AddAllowedInteractor { interactor } => {
            let owner = Contract::owner()
                .parse::<Address>()
                .map_err(|err| ContractError::ParseError(err.to_string()))?;

            actions::interactors::add(current_state, owner, interactor).await
        }
        Action::RemoveAllowedInteractor { interactor } => {
            let owner = Contract::owner()
                .parse::<Address>()
                .map_err(|err| ContractError::ParseError(err.to_string()))?;

            actions::interactors::remove(current_state, owner, interactor).await
        }
    }
}
