use serde::Serialize;

#[derive(Serialize)]
pub enum ContractError {
    RuntimeError(String),
    InvalidAddress(String),
}
