#[allow(dead_code)]
mod constants;
mod contracts;
mod event;
mod validator;

use cosmwasm_std::{attr, coin, Attribute, Binary, Uint128};
use ethers::{
    prelude::parse_log, providers::Middleware, signers::Signer, types::TransactionReceipt,
};
use ibcx_test_utils::addr;
use osmosis_test_tube::{
    osmosis_std::types::cosmwasm::wasm::v1::MsgExecuteContractResponse, Account, Module,
    OsmosisTestApp, Wasm,
};

use hpl_interface::{
    core::mailbox::{self, DispatchMsg},
    igp::oracle::RemoteGasDataConfig,
    types::{bech32_decode, bech32_encode, bech32_to_h256, AggregateMetadata},
};
use test_tube::{ExecuteResponse, Runner};

use crate::{
    constants::*,
    contracts::{
        cw,
        eth::{
            self,
            mailbox::{DispatchFilter, DispatchIdFilter},
        },
    },
    event::{parse_dispatch_from_res, parse_dispatch_id_from_res},
    validator::TestValidators,
};

fn sorted(mut attrs: Vec<Attribute>) -> Vec<Attribute> {
    attrs.sort_by(|a, b| a.key.cmp(&b.key));
    attrs
}

async fn send_msg_cw_to_evm<'a, M, S, R>(
    from: &cw::Env<'a, R>,
    to: &eth::Env<M, S>,
) -> eyre::Result<TransactionReceipt>
where
    M: Middleware + 'static,
    S: Signer + 'static,
    R: Runner<'a>,
{
    let mut receiver = [0u8; 32];
    receiver[12..].copy_from_slice(&to.core.msg_receiver.address().0);
    let _sender = bech32_decode(from.acc_tester.address().as_str())?;
    let msg_body = b"hello world";

    // dispatch
    let dispatch_res = Wasm::new(from.app).execute(
        &from.core.mailbox,
        &mailbox::ExecuteMsg::Dispatch(DispatchMsg {
            dest_domain: DOMAIN_EVM,
            recipient_addr: receiver.into(),
            msg_body: msg_body.into(),
            hook: None,
            metadata: None,
        }),
        &[coin(56_000_000, "uosmo")],
        &from.acc_tester,
    )?;

    let dispatch = parse_dispatch_from_res(&dispatch_res.events);
    let _dispatch_id = parse_dispatch_id_from_res(&dispatch_res.events);

    let process_tx = to
        .core
        .mailbox
        .process(vec![].into(), Binary::from(dispatch.message).0.into());
    let process_tx_res = process_tx.send().await?.await?.unwrap();

    Ok(process_tx_res)
}

async fn send_msg_evm_to_cw<'a, M, S, R>(
    from: &eth::Env<M, S>,
    to: &cw::Env<'a, R>,
) -> eyre::Result<ExecuteResponse<MsgExecuteContractResponse>>
where
    M: Middleware + 'static,
    S: Signer + 'static,
    R: Runner<'a>,
{
    // prepare message arguments
    let sender = bech32_encode("osmo", from.acc_owner.address().as_bytes())?;
    let receiver = bech32_to_h256(&to.core.msg_receiver)?;
    let msg_body = b"hello world";

    // dispatch
    let dispatch_tx_call = from
        .core
        .mailbox
        .dispatch(DOMAIN_OSMO, receiver, msg_body.into());
    let dispatch_res = dispatch_tx_call.send().await?.await?.unwrap();

    let dispatch: DispatchFilter = parse_log(dispatch_res.logs[0].clone())?;
    let dispatch_id: DispatchIdFilter = parse_log(dispatch_res.logs[1].clone())?;

    // generate ism metadata
    let multisig_ism_metadata = to.get_validator_set(DOMAIN_EVM)?.make_metadata(
        from.core.mailbox.address(),
        from.core.mailbox.root().await?,
        from.core.mailbox.count().await? - 1,
        dispatch_id.message_id,
        true,
    )?;

    let aggregate_ism_metadata = AggregateMetadata::new(vec![(
        addr(&to.core.default_ism),
        multisig_ism_metadata.into(),
    )]);

    // process
    let process_res = Wasm::new(to.app).execute(
        &to.core.mailbox,
        &mailbox::ExecuteMsg::Process {
            metadata: aggregate_ism_metadata.into(),
            message: dispatch.message.to_vec().into(),
        },
        &[],
        &to.acc_owner,
    )?;
    let process_recv_evt = process_res
        .events
        .iter()
        .find(|v| v.ty == "wasm-mailbox_msg_received")
        .unwrap();

    assert_eq!(
        process_recv_evt.attributes,
        sorted(vec![
            Attribute {
                key: "_contract_address".to_string(),
                value: to.core.msg_receiver.clone(),
            },
            attr("sender", sender),
            attr(
                "origin",
                from.core.mailbox.local_domain().await?.to_string()
            ),
            attr("body", std::str::from_utf8(msg_body)?),
        ]),
    );

    Ok(process_res)
}

#[tokio::test]
#[ignore]
async fn test_mailbox_cw_to_evm() -> eyre::Result<()> {
    // init Osmosis env
    let osmo_app = OsmosisTestApp::new();
    let osmo = cw::setup_env(
        &osmo_app,
        |app, coins| app.init_account(coins).unwrap(),
        None::<&str>,
        "osmo",
        DOMAIN_OSMO,
        &[TestValidators::new(DOMAIN_EVM, 5, 3)],
        &[RemoteGasDataConfig {
            remote_domain: DOMAIN_EVM,
            token_exchange_rate: Uint128::from(10u128.pow(4)),
            gas_price: Uint128::from(10u128.pow(9)),
        }],
    )?;

    // init Anvil env
    let anvil1 = eth::setup_env(DOMAIN_EVM).await?;

    let _ = send_msg_cw_to_evm(&osmo, &anvil1).await?;

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_mailbox_evm_to_cw() -> eyre::Result<()> {
    // init Osmosis env
    let osmo_app = OsmosisTestApp::new();
    let osmo = cw::setup_env(
        &osmo_app,
        |app, coins| app.init_account(coins).unwrap(),
        None::<&str>,
        "osmo",
        DOMAIN_OSMO,
        &[TestValidators::new(DOMAIN_EVM, 5, 3)],
        &[RemoteGasDataConfig {
            remote_domain: DOMAIN_EVM,
            token_exchange_rate: Uint128::from(10u128.pow(4)),
            gas_price: Uint128::from(10u128.pow(9)),
        }],
    )?;

    // init Anvil env
    let anvil1 = eth::setup_env(DOMAIN_EVM).await?;

    let _ = send_msg_evm_to_cw(&anvil1, &osmo).await?;

    Ok(())
}
