use bundlr_contracts_shared::{Address, TransactionId};
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub enum ContractError {
    NominatedValidatorCannotLeave(Address),
    InvalidValidator(Address),
    ParseError(String),
    RuntimeError(String),
    TransferFailed,
    UpdateEpochBlocked,
    AlreadyProposed(TransactionId),
    TooManyProposals,
    InvalidStake,
    AlreadyJoined,
    InvalidTransactionId,
    ProposalExpired,
    VotingClosed,
    AlreadyVoted,
}
