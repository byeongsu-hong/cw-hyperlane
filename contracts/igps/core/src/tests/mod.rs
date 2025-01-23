use cosmwasm_std::{
    from_json,
    testing::{mock_info, MockApi, MockQuerier, MockStorage},
    Addr, Coin, Deps, DepsMut, Empty, Env, HexBinary, MessageInfo, OwnedDeps, Response, Uint128,
};
use hpl_interface::{
    hook::PostDispatchMsg,
    igp::{
        core::{
            ExecuteMsg, GasOracleConfig, IgpQueryMsg, InstantiateMsg, QueryMsg,
            QuoteGasPaymentResponse,
        },
        oracle::{GetExchangeRateAndGasPriceResponse, IgpGasOracleQueryMsg},
    },
    router::{DomainRouteSet, RouterMsg},
};
use serde::de::DeserializeOwned;

use crate::{
    contract::{execute, instantiate, query},
    ContractError,
};

mod contract;

pub struct IGP {
    pub deps: OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
    pub env: Env,
}

impl IGP {
    pub fn new(deps: OwnedDeps<MockStorage, MockApi, MockQuerier>, env: Env) -> Self {
        Self { deps, env }
    }

    pub fn with_env(&mut self, env: Env) {
        self.env = env
    }

    pub fn init(
        &mut self,
        sender: &Addr,
        hrp: &str,
        owner: &Addr,
        gas_token: &str,
        beneficiary: &Addr,
    ) -> Result<Response, ContractError> {
        instantiate(
            self.deps.as_mut(),
            self.env.clone(),
            mock_info(sender.as_str(), &[]),
            InstantiateMsg {
                hrp: hrp.to_string(),
                owner: owner.to_string(),
                gas_token: gas_token.to_string(),
                beneficiary: beneficiary.to_string(),
                default_gas_usage: Uint128::from(250_000u128),
            },
        )
    }

    fn execute(&mut self, info: MessageInfo, msg: ExecuteMsg) -> Result<Response, ContractError> {
        execute(self.deps.as_mut(), self.env.clone(), info, msg)
    }

    fn query<T: DeserializeOwned>(&self, msg: QueryMsg) -> Result<T, ContractError> {
        query(self.deps.as_ref(), self.env.clone(), msg)
            .map(|v| from_json::<T>(&v))?
            .map_err(|e| e.into())
    }

    pub fn deps_mut(&mut self) -> DepsMut {
        self.deps.as_mut()
    }

    pub fn deps_ref(&self) -> Deps {
        self.deps.as_ref()
    }

    pub fn set_gas_oracles(
        &mut self,
        sender: &Addr,
        configs: Vec<GasOracleConfig>,
    ) -> Result<Response, ContractError> {
        self.execute(
            mock_info(sender.as_str(), &[]),
            ExecuteMsg::Router(RouterMsg::<Addr>::SetRoutes {
                set: configs
                    .into_iter()
                    .map(|v| DomainRouteSet {
                        domain: v.remote_domain,
                        route: Some(Addr::unchecked(v.gas_oracle)),
                    })
                    .collect(),
            }),
        )
    }

    pub fn set_beneficiary(
        &mut self,
        sender: &Addr,
        beneficiary: &Addr,
    ) -> Result<Response, ContractError> {
        self.execute(
            mock_info(sender.as_str(), &[]),
            ExecuteMsg::SetBeneficiary {
                beneficiary: beneficiary.to_string(),
            },
        )
    }

    pub fn claim(&mut self, sender: &Addr) -> Result<Response, ContractError> {
        self.execute(mock_info(sender.as_str(), &[]), ExecuteMsg::Claim {})
    }

    pub fn pay_for_gas(
        &mut self,
        sender: &Addr,
        funds: &[Coin],
        message_id: &HexBinary,
        dest_domain: u32,
        gas_amount: u128,
        refund_address: &Addr,
    ) -> Result<Response, ContractError> {
        self.execute(
            mock_info(sender.as_str(), funds),
            ExecuteMsg::PayForGas {
                message_id: message_id.clone(),
                dest_domain,
                gas_amount: gas_amount.into(),
                refund_address: refund_address.to_string(),
            },
        )
    }

    pub fn post_dispatch(
        &mut self,
        sender: &Addr,
        metadata: HexBinary,
        message: HexBinary,
        funds: Vec<Coin>,
    ) -> Result<Response, ContractError> {
        self.execute(
            mock_info(sender.as_str(), &funds),
            ExecuteMsg::PostDispatch(PostDispatchMsg { metadata, message }),
        )
    }

    pub fn get_quote_gas_payment(
        &self,
        dest_domain: u32,
        gas_amount: u128,
    ) -> Result<QuoteGasPaymentResponse, ContractError> {
        self.query(
            IgpQueryMsg::QuoteGasPayment {
                dest_domain,
                gas_amount: gas_amount.into(),
            }
            .wrap(),
        )
    }

    pub fn get_exchange_rate_and_gas_price(
        &self,
        dest_domain: u32,
    ) -> Result<GetExchangeRateAndGasPriceResponse, ContractError> {
        self.query(QueryMsg::Oracle(
            IgpGasOracleQueryMsg::GetExchangeRateAndGasPrice { dest_domain },
        ))
    }
}
