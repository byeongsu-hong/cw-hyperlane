use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, HexBinary, Uint128, Uint256};

use crate::{
    hook::{HookQueryMsg, PostDispatchMsg},
    ownable::{OwnableMsg, OwnableQueryMsg},
    router::{RouterMsg, RouterQuery},
    Order,
};

use super::oracle::IgpGasOracleQueryMsg;

#[cw_serde]
pub struct InstantiateMsg {
    pub hrp: String,
    pub owner: String,
    pub gas_token: String,
    pub beneficiary: String,
    pub default_gas_usage: Uint128,
}

#[cw_serde]
pub struct GasOracleConfig {
    pub remote_domain: u32,
    pub gas_oracle: String,
}

impl From<(u32, String)> for GasOracleConfig {
    fn from((remote_domain, gas_oracle): (u32, String)) -> Self {
        Self {
            remote_domain,
            gas_oracle,
        }
    }
}

impl From<(u32, Addr)> for GasOracleConfig {
    fn from((remote_domain, gas_oracle): (u32, Addr)) -> Self {
        Self {
            remote_domain,
            gas_oracle: gas_oracle.to_string(),
        }
    }
}

#[cw_serde]
pub enum ExecuteMsg {
    // overrides
    Ownable(OwnableMsg),
    Router(RouterMsg<Addr>),
    PostDispatch(PostDispatchMsg),

    // base
    SetDefaultGas {
        gas: u128,
    },
    SetGasForDomain {
        config: Vec<(u32, u128)>,
    },
    UnsetGasForDomain {
        domains: Vec<u32>,
    },

    SetBeneficiary {
        beneficiary: String,
    },
    PayForGas {
        message_id: HexBinary,
        dest_domain: u32,
        gas_amount: Uint256,
        refund_address: String,
    },
    Claim {},
}

#[cw_serde]
#[derive(QueryResponses)]
#[query_responses(nested)]
pub enum QueryMsg {
    // overrides
    Ownable(OwnableQueryMsg),
    Hook(HookQueryMsg),
    Router(RouterQuery<Addr>),
    Oracle(IgpGasOracleQueryMsg),

    // base
    Igp(IgpQueryMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum IgpQueryMsg {
    #[returns(DefaultGasResponse)]
    DefaultGas {},

    #[returns(GasForDomainResponse)]
    GasForDomain { domains: Vec<u32> },

    #[returns(GasForDomainResponse)]
    ListGasForDomains {
        offset: Option<u32>,
        limit: Option<u32>,
        order: Option<Order>,
    },

    #[returns(BeneficiaryResponse)]
    Beneficiary {},

    #[returns(QuoteGasPaymentResponse)]
    QuoteGasPayment {
        dest_domain: u32,
        gas_amount: Uint256,
    },
}

impl IgpQueryMsg {
    pub fn wrap(self) -> QueryMsg {
        QueryMsg::Igp(self)
    }
}

#[cw_serde]
pub struct DefaultGasResponse {
    pub gas: u128,
}

#[cw_serde]
pub struct GasForDomainResponse {
    pub gas: Vec<(u32, u128)>,
}

#[cw_serde]
pub struct BeneficiaryResponse {
    pub beneficiary: String,
}

#[cw_serde]
pub struct QuoteGasPaymentResponse {
    pub gas_needed: Uint256,
}

#[cfg(test)]
mod test {
    use cosmwasm_std::HexBinary;

    use super::*;
    use crate::{
        hook::{ExpectedHookQueryMsg, PostDispatchMsg, QuoteDispatchMsg},
        msg_checker,
    };

    #[test]
    fn test_hook_interface() {
        let _checked: ExecuteMsg = msg_checker(
            PostDispatchMsg {
                metadata: HexBinary::default(),
                message: HexBinary::default(),
            }
            .wrap(),
        );

        let _checked: QueryMsg = msg_checker(ExpectedHookQueryMsg::Hook(HookQueryMsg::Mailbox {}));
        let _checked: QueryMsg = msg_checker(
            QuoteDispatchMsg {
                metadata: HexBinary::default(),
                message: HexBinary::default(),
            }
            .request(),
        );
    }
}
