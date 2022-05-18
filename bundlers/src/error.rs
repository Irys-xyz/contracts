use bundlr_contracts_shared::{Address, BlockHeight};
use serde::Serialize;

#[derive(Serialize)]
pub enum ContractError {
    AlreadyLeaving(Address, BlockHeight),
    InvalidBundler(Address),
    ParseError(String),
    RuntimeError(String),
    TransferFailed,
    WithdrawBlocked,
}
