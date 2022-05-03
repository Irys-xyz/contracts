use crate::action::{Action, ActionResult};
use crate::actions;
use crate::state::State;

pub async fn handle(current_state: State, action: Action) -> ActionResult {
    match action {
        Action::Validators => actions::queries::validators(current_state),
        Action::NominatedValidators => actions::queries::nominated_validators(current_state),
        Action::Stake => actions::queries::stake(current_state),
        Action::Token => actions::queries::token(current_state),
        Action::Bundler => actions::queries::bundler(current_state),
        Action::BundlersContract => actions::queries::bundlers_contract(current_state),
        Action::Epoch => actions::queries::epoch(current_state),
        Action::EpochDuration => actions::queries::epoch_duration(current_state),
        Action::Join => actions::join(current_state).await,
        Action::Leave => actions::leave(current_state).await,
        Action::UpdateEpoch => actions::update_epoch(current_state).await,
        Action::ProposeSlash => todo!(),
        Action::VoteSlash => todo!(),
        Action::SyncSlashed => todo!(),
    }
}
