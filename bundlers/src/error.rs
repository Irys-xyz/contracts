use bundlr_contracts_shared::{Address, BlockHeight};
use serde::Serialize;

#[derive(Serialize)]
pub enum ContractError {
    AlreadyJoined(Address),
    AlreadyLeaving(Address, BlockHeight),
    AlreadyInteractor(Address),
    Forbidden,
    InvalidBundler(Address),
    InvalidInteractor(Address),
    ParseError(String),
    RuntimeError(String),
    TransferFailed,
    WithdrawBlocked,
}
