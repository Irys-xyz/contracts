use bundlr_contracts_shared::Address;
use serde::Serialize;

#[derive(Serialize)]
pub enum ContractError {
    NominatedValidatorCannotLeave(Address),
    InvalidValidator(Address),
    InvalidCaller,
    ParseError(String),
    RuntimeError(String),
    TransferFailed,
    UpdateEpochBlocked,
}
