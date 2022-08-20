mod epoch;
mod join;
mod leave;

pub mod queries;
pub mod slashing;

#[cfg(feature = "js-runtime")]
pub use epoch::update_epoch;
#[cfg(feature = "js-runtime")]
pub use join::join;
#[cfg(feature = "js-runtime")]
pub use leave::leave;
