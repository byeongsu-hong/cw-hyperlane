use std::str::FromStr;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coins, to_binary, BankMsg, Deps, DepsMut, Env, Event, MessageInfo, QuerierWrapper,
    QueryResponse, Response, Storage, Uint128, Uint256,
};

use cw_utils::PaymentError;
use hpl_interface::{
    igp_core::{
        ExecuteMsg, GetExchangeRateAndGasPriceResponse, InstantiateMsg, MigrateMsg, QueryMsg,
        QuoteGasPaymentResponse,
    },
    igp_gas_oracle,
};

use crate::{
    error::ContractError,
    state::{BENEFICIARY, GAS_ORACLE, GAS_TOKEN},
    CONTRACT_NAME, CONTRACT_VERSION,
};

const TOKEN_EXCHANGE_RATE_SCALE: u128 = 10_000_000_000;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    hpl_ownable::OWNER.save(deps.storage, &deps.api.addr_validate(&msg.owner)?)?;
    BENEFICIARY.save(deps.storage, &deps.api.addr_validate(&msg.beneficiary)?)?;
    GAS_TOKEN.save(deps.storage, &msg.gas_token)?;

    Ok(Response::new().add_event(
        Event::new("init-igp-core")
            .add_attribute("owner", msg.owner)
            .add_attribute("creator", info.sender)
            .add_attribute("beneficiary", msg.beneficiary),
    ))
}

fn quote_gas_price(
    storage: &dyn Storage,
    querier: &QuerierWrapper,
    dest_domain: u32,
    gas_amount: Uint256,
) -> Result<Uint256, ContractError> {
    let gas_oracle = GAS_ORACLE
        .may_load(storage, dest_domain)?
        .ok_or(ContractError::GasOracleNotFound {})?;

    let gas_price_resp: igp_gas_oracle::GetExchangeRateAndGasPriceResponse = querier
        .query_wasm_smart(
            gas_oracle,
            &igp_gas_oracle::QueryMsg::GetExchangeRateAndGasPrice { dest_domain },
        )?;

    let dest_gas_cost = gas_amount * Uint256::from(gas_price_resp.gas_price);
    let gas_needed = (dest_gas_cost * Uint256::from(gas_price_resp.exchange_rate))
        / Uint256::from(TOKEN_EXCHANGE_RATE_SCALE);

    Ok(gas_needed)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Ownership(msg) => Ok(hpl_ownable::handle(deps, env, info, msg)?),
        ExecuteMsg::SetGasOracles { configs } => {
            if info.sender != hpl_ownable::OWNER.load(deps.storage)? {
                return Err(ContractError::Unauthorized {});
            }

            let mut domains = vec![];
            for c in configs {
                domains.push(c.remote_domain.to_string());

                GAS_ORACLE.save(
                    deps.storage,
                    c.remote_domain,
                    &deps.api.addr_validate(&c.gas_oracle)?,
                )?;
            }

            Ok(Response::new().add_event(
                Event::new("set-gas-oracles")
                    .add_attribute("owner", info.sender)
                    .add_attribute("domains", domains.join(",")),
            ))
        }
        ExecuteMsg::SetBeneficiary { beneficiary } => {
            if info.sender != hpl_ownable::OWNER.load(deps.storage)? {
                return Err(ContractError::Unauthorized {});
            }

            BENEFICIARY.save(deps.storage, &deps.api.addr_validate(&beneficiary)?)?;

            Ok(Response::new().add_event(
                Event::new("set-beneficiary")
                    .add_attribute("owner", info.sender)
                    .add_attribute("beneficiary", beneficiary),
            ))
        }
        ExecuteMsg::Claim {} => {
            if info.sender != BENEFICIARY.load(deps.storage)? {
                return Err(ContractError::Unauthorized {});
            }

            Ok(Response::new().add_event(Event::new("claim")))
        }

        ExecuteMsg::PayForGas {
            message_id,
            dest_domain,
            gas_amount,
            refund_address,
        } => {
            let gas_token = GAS_TOKEN.load(deps.storage)?;
            let received = Uint256::from(cw_utils::must_pay(&info, &gas_token)?);
            let gas_needed = quote_gas_price(deps.storage, &deps.querier, dest_domain, gas_amount)?;
            if received < gas_needed {
                return Err(PaymentError::NonPayable {}.into());
            }

            let payment_gap = Uint128::from_str(&(received - gas_needed).to_string())?;

            let refund_msg = BankMsg::Send {
                to_address: refund_address,
                amount: coins(payment_gap.u128(), &gas_token),
            };

            Ok(Response::new().add_message(refund_msg).add_event(
                Event::new("pay-for-gas")
                    .add_attribute("sender", info.sender)
                    .add_attribute("message_id", message_id.to_base64())
                    .add_attribute("gas_amount", gas_amount)
                    .add_attribute("gas_required", gas_needed),
            ))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    match msg {
        QueryMsg::QuoteGasPayment {
            dest_domain,
            gas_amount,
        } => {
            let gas_needed = quote_gas_price(deps.storage, &deps.querier, dest_domain, gas_amount)?;

            Ok(to_binary(&QuoteGasPaymentResponse { gas_needed })?)
        }
        QueryMsg::GetExchangeRateAndGasPrice { dest_domain } => {
            let gas_oracle = GAS_ORACLE
                .may_load(deps.storage, dest_domain)?
                .ok_or(ContractError::GasOracleNotFound {})?;

            let gas_price_resp: igp_gas_oracle::GetExchangeRateAndGasPriceResponse =
                deps.querier.query_wasm_smart(
                    gas_oracle,
                    &igp_gas_oracle::QueryMsg::GetExchangeRateAndGasPrice { dest_domain },
                )?;

            Ok(to_binary(&GetExchangeRateAndGasPriceResponse {
                gas_price: gas_price_resp.gas_price,
                exchange_rate: gas_price_resp.exchange_rate,
            })?)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}
