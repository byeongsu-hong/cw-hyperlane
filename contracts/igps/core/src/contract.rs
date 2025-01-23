#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    ensure, Deps, DepsMut, Empty, Env, Event, MessageInfo, QueryResponse, Response,
};

use hpl_interface::hook::HookQueryMsg;
use hpl_interface::igp::core::{ExecuteMsg, IgpQueryMsg, InstantiateMsg, QueryMsg};
use hpl_interface::igp::oracle::IgpGasOracleQueryMsg;
use hpl_interface::to_binary;

use crate::{
    ContractError, BENEFICIARY, CONTRACT_NAME, CONTRACT_VERSION, DEFAULT_GAS_USAGE, GAS_TOKEN, HRP,
};

fn new_event(name: &str) -> Event {
    Event::new(format!("hpl_igp_core::{}", name))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // check hrp is lowercase
    ensure!(
        msg.hrp.chars().all(|v| v.is_lowercase()),
        ContractError::invalid_config("hrp must be lowercase")
    );

    // check gas token exists
    deps.querier.query_supply(&msg.gas_token).map_err(|e| {
        ContractError::invalid_config(&format!("gas_token {} does not exist: {e}", msg.gas_token,))
    })?;

    let owner = deps.api.addr_validate(&msg.owner)?;
    let beneficiary = deps.api.addr_validate(&msg.beneficiary)?;

    hpl_ownable::initialize(deps.storage, &owner)?;

    BENEFICIARY.save(deps.storage, &beneficiary)?;

    GAS_TOKEN.save(deps.storage, &msg.gas_token)?;
    HRP.save(deps.storage, &msg.hrp)?;
    DEFAULT_GAS_USAGE.save(deps.storage, &msg.default_gas_usage.into())?;

    Ok(Response::new().add_event(
        new_event("initialize")
            .add_attribute("sender", info.sender)
            .add_attribute("owner", msg.owner)
            .add_attribute("beneficiary", msg.beneficiary)
            .add_attribute("default_gas", msg.default_gas_usage.to_string()),
    ))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use crate::execute;

    match msg {
        ExecuteMsg::Ownable(msg) => Ok(hpl_ownable::handle(deps, env, info, msg)?),
        ExecuteMsg::Router(msg) => Ok(hpl_router::handle(deps, env, info, msg)?),
        ExecuteMsg::PostDispatch(msg) => Ok(execute::post_dispatch(deps, info, msg)?),

        ExecuteMsg::SetDefaultGas { gas } => execute::set_default_gas(deps, info, gas),
        ExecuteMsg::SetGasForDomain { config } => execute::set_gas_for_domain(deps, info, config),
        ExecuteMsg::UnsetGasForDomain { domains } => {
            execute::unset_gas_for_domain(deps, info, domains)
        }

        ExecuteMsg::SetBeneficiary { beneficiary } => {
            execute::set_beneficiary(deps, info, beneficiary)
        }
        ExecuteMsg::Claim {} => execute::claim(deps, env, info),

        ExecuteMsg::PayForGas {
            message_id,
            dest_domain,
            gas_amount,
            refund_address,
        } => execute::pay_for_gas(
            &deps,
            info,
            message_id,
            dest_domain,
            gas_amount,
            refund_address,
        ),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    use crate::query::*;

    match msg {
        QueryMsg::Ownable(msg) => Ok(hpl_ownable::handle_query(deps, env, msg)?),
        QueryMsg::Router(msg) => Ok(hpl_router::handle_query(deps, env, msg)?),

        QueryMsg::Hook(msg) => match msg {
            HookQueryMsg::QuoteDispatch(msg) => to_binary(quote_dispatch(deps, msg)),
            HookQueryMsg::Mailbox {} => to_binary(get_mailbox(deps)),
        },
        QueryMsg::Oracle(msg) => match msg {
            IgpGasOracleQueryMsg::GetExchangeRateAndGasPrice { dest_domain } => {
                to_binary(get_exchange_rate_and_gas_price(deps, dest_domain))
            }
        },
        QueryMsg::Igp(msg) => match msg {
            IgpQueryMsg::DefaultGas {} => to_binary(get_default_gas(deps)),
            IgpQueryMsg::GasForDomain { domains } => to_binary(get_gas_for_domain(deps, domains)),
            IgpQueryMsg::ListGasForDomains {
                offset,
                limit,
                order,
            } => to_binary(list_gas_for_domains(deps, offset, limit, order)),

            IgpQueryMsg::Beneficiary {} => to_binary(get_beneficiary(deps)),

            IgpQueryMsg::QuoteGasPayment {
                dest_domain,
                gas_amount,
            } => to_binary(quote_gas_payment(deps, dest_domain, gas_amount)),
        },
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, ContractError> {
    hpl_utils::migrate(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}
