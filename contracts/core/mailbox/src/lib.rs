pub mod contract;
mod error;
mod event;
pub mod execute;
pub mod query;
mod state;

pub use crate::error::ContractError;

pub const MAILBOX_VERSION: u8 = 3;

// version info for migration info
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
