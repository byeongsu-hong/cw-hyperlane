#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_binary, to_binary, to_vec, Binary, CanonicalAddr, ContractResult, CosmosMsg, Deps,
    DepsMut, Empty, Env, MessageInfo, QueryRequest, Response, StdError, StdResult, SystemResult,
};
use cw2::set_contract_version;
use hpl_interface::{
    mailbox,
    multicall::{AggregateResponse, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
};

use crate::{
    error::ContractError,
    state::{Config, CONFIG},
    CONTRACT_NAME, CONTRACT_VERSION,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        owner: deps.api.addr_validate(&msg.owner)?,
        mailbox: deps.api.addr_validate(&msg.mailbox)?,
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", config.owner))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        Aggregate { req } => {
            let config = CONFIG.load(deps.storage)?;

            assert_eq!(config.owner, info.sender, "not an owner");

            let resp = Response::new().add_messages(req);

            Ok(resp)
        }

        Handle(mailbox::HandleMsg { sender, body, .. }) => {
            let config = CONFIG.load(deps.storage)?;
            assert_eq!(config.mailbox, info.sender, "not a mailbox");

            let sender = deps
                .api
                .addr_humanize(&CanonicalAddr::from(Binary::from(sender)))?;
            assert_eq!(config.owner, sender, "not an owner");

            let msgs: Vec<CosmosMsg> = from_binary(&Binary::from(body))?;
            let resp = Response::new().add_messages(msgs);

            Ok(resp)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    use QueryMsg::*;

    match msg {
        AggregateStatic { req } => {
            let resps = req
                .into_iter()
                .map(|call| {
                    let raw = to_vec(&QueryRequest::<Empty>::Stargate {
                        path: call.path,
                        data: call.data,
                    })?;

                    match deps.querier.raw_query(&raw) {
                        SystemResult::Err(system_err) => Err(StdError::generic_err(format!(
                            "Querier system error: {system_err}",
                        ))),
                        SystemResult::Ok(ContractResult::Err(contract_err)) => {
                            Err(StdError::generic_err(format!(
                                "Querier contract error: {contract_err}",
                            )))
                        }
                        SystemResult::Ok(ContractResult::Ok(value)) => Ok(value),
                    }
                })
                .collect::<StdResult<Vec<_>>>()?;

            Ok(to_binary(&AggregateResponse { resp: resps })?)
        }
    }
}
