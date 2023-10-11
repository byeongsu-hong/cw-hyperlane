use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, PartialEq, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    ParseReplyError(#[from] cw_utils::ParseReplyError),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("WrongLength")]
    WrongLength {},

    #[error("InvalidTokenOption")]
    InvalidTokenOption,

    #[error("InvalidReplyId")]
    InvalidReplyId,

    #[error("InvalidReceiveMsg")]
    InvalidReceiveMsg,

    #[error("NoRouter domain:{domain:?}")]
    NoRouter { domain: u32 },
}
