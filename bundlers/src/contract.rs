use crate::action::{Action, ActionResult};
use crate::actions;
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
    }
}
