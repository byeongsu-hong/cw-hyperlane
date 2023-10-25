use cosmwasm_std::{RecoverPubkeyError, StdError, VerificationError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    VerificationError(#[from] VerificationError),

    #[error("{0}")]
    RecoverPubkeyError(#[from] RecoverPubkeyError),

    #[error("unauthorized")]
    Unauthorized,

    #[error("wrong length")]
    WrongLength,

    #[error("invalid pubkey")]
    InvalidPubKey,

    #[error("duplicate validator")]
    ValidatorDuplicate,

    #[error("validator not exists")]
    ValidatorNotExist,
}
