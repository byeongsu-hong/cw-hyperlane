#[allow(dead_code)]
mod constants;
mod contracts;
mod event;
mod validator;

use std::collections::BTreeMap;

use cosmwasm_std::{Event, Uint128};
use hpl_interface::{igp::oracle::RemoteGasDataConfig, warp};
use osmosis_test_tube::{
    osmosis_std::types::osmosis::tokenfactory::{
        self,
        v1beta1::{MsgChangeAdmin, MsgCreateDenom},
    },
    OsmosisTestApp, TokenFactory,
};
use rstest::rstest;
use test_tube::{Account, Module, Wasm};

use crate::{constants::*, contracts::cw, validator::TestValidators};

fn wasm_events(events: Vec<Event>) -> BTreeMap<String, BTreeMap<String, String>> {
    events
        .into_iter()
        .filter(|v| v.ty.starts_with("wasm-"))
        .map(|v| {
            (
                v.ty,
                v.attributes
                    .into_iter()
                    .map(|x| (x.key, x.value))
                    .collect::<BTreeMap<_, _>>(),
            )
        })
        .collect::<BTreeMap<_, _>>()
}

#[tokio::test]
async fn test_cw20_colleteral() -> eyre::Result<()> {
    let osmo_app = OsmosisTestApp::new();
    let osmo = cw::setup_env(
        &osmo_app,
        |app, coins| app.init_account(coins).unwrap(),
        Some("../artifacts/"),
        "osmo",
        DOMAIN_OSMO,
        &[TestValidators::new(DOMAIN_EVM, 5, 3)],
        &[RemoteGasDataConfig {
            remote_domain: DOMAIN_EVM,
            token_exchange_rate: Uint128::from(10u128.pow(4)),
            gas_price: Uint128::from(10u128.pow(9)),
        }],
    )?;

    let wasm = Wasm::new(&osmo_app);

    // deploy new cw20 token
    let mock_token = cw::instantiate(
        &wasm,
        osmo.codes.cw20_base,
        &osmo.acc_deployer,
        "cw20-base",
        &hpl_interface::warp::cw20::Cw20InitMsg {
            name: "denomdenom".into(),
            symbol: "denomdenom".into(),
            decimals: 6,
            initial_balances: vec![],
            mint: Some(cw20::MinterResponse {
                minter: osmo.acc_owner.address(),
                cap: None,
            }),
            marketing: None,
        },
    )
    .data
    .address;

    // deploy warp route with deployed cw20 token
    let warp_resp = cw::deploy_warp_route_collateral(
        &wasm,
        &osmo.acc_owner,
        &osmo.acc_deployer,
        &osmo.core.mailbox,
        "osmo",
        &osmo.codes,
        mock_token.clone(),
    );

    let events = wasm_events(warp_resp.events);
    assert_eq!(
        events["wasm-hpl_warp_cw20::instantiate"]["denom"],
        mock_token
    );

    // move cw20 token's minter to warp route contract
    wasm.execute(
        &mock_token,
        &cw20::Cw20ExecuteMsg::UpdateMinter {
            new_minter: Some(warp_resp.data.address),
        },
        &[],
        &osmo.acc_owner,
    )?;

    // ready to mint / burn!

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_cw20_bridged() -> eyre::Result<()> {
    let osmo_app = OsmosisTestApp::new();
    let osmo = cw::setup_env(
        &osmo_app,
        |app, coins| app.init_account(coins).unwrap(),
        Some("../artifacts/"),
        "osmo",
        DOMAIN_OSMO,
        &[TestValidators::new(DOMAIN_EVM, 5, 3)],
        &[RemoteGasDataConfig {
            remote_domain: DOMAIN_EVM,
            token_exchange_rate: Uint128::from(10u128.pow(4)),
            gas_price: Uint128::from(10u128.pow(9)),
        }],
    )?;

    let wasm = Wasm::new(&osmo_app);

    let warp_resp = cw::deploy_warp_route_bridged(
        &wasm,
        &osmo.acc_owner,
        &osmo.acc_deployer,
        &osmo.core.mailbox,
        "osmo",
        &osmo.codes,
        "denomdenom".into(),
        warp::TokenType::CW20 {
            contract: "".into(),
        },
    );

    let events = wasm_events(warp_resp.events);
    assert_eq!(
        events["wasm-hpl_warp_cw20::instantiate"]["denom"],
        "denomdenom"
    );

    let token_addr_from_evt = &events["wasm-hpl_warp_cw20::reply-init"]["new_token"];
    let token_info: cw20::TokenInfoResponse =
        wasm.query(token_addr_from_evt, &cw20::Cw20QueryMsg::TokenInfo {})?;
    assert_eq!(token_info.name, "denomdenom");

    let minter_info: Option<cw20::MinterResponse> =
        wasm.query(token_addr_from_evt, &cw20::Cw20QueryMsg::Minter {})?;
    assert_eq!(minter_info.unwrap().minter, warp_resp.data.address);

    Ok(())
}

#[rstest]
#[case("utest")]
#[tokio::test]
async fn test_native_collateral(#[case] denom: &str) -> eyre::Result<()> {
    let osmo_app = OsmosisTestApp::new();
    let osmo = cw::setup_env(
        &osmo_app,
        |app, coins| app.init_account(coins).unwrap(),
        Some("../artifacts/"),
        "osmo",
        DOMAIN_OSMO,
        &[TestValidators::new(DOMAIN_EVM, 5, 3)],
        &[RemoteGasDataConfig {
            remote_domain: DOMAIN_EVM,
            token_exchange_rate: Uint128::from(10u128.pow(4)),
            gas_price: Uint128::from(10u128.pow(9)),
        }],
    )?;

    let wasm = Wasm::new(&osmo_app);
    let tf = TokenFactory::new(&osmo_app);

    let mock_token = tf
        .create_denom(
            MsgCreateDenom {
                sender: osmo.acc_deployer.address(),
                subdenom: denom.to_string(),
            },
            &osmo.acc_deployer,
        )?
        .data
        .new_token_denom;

    let warp_resp = cw::deploy_warp_route_collateral(
        &wasm,
        &osmo.acc_owner,
        &osmo.acc_deployer,
        &osmo.core.mailbox,
        "osmo",
        &osmo.codes,
        mock_token.clone(),
    );

    let events = wasm_events(warp_resp.events);
    assert_eq!(
        events["wasm-hpl_warp_native::instantiate"]["denom"],
        mock_token
    );

    let resp = tf.query_denom_authority_metadata(
        &tokenfactory::v1beta1::QueryDenomAuthorityMetadataRequest {
            denom: mock_token.clone(),
        },
    )?;
    assert_eq!(
        resp.authority_metadata.unwrap().admin,
        osmo.acc_deployer.address()
    );

    tf.change_admin(
        MsgChangeAdmin {
            sender: osmo.acc_deployer.address(),
            denom: mock_token,
            new_admin: warp_resp.data.address,
        },
        &osmo.acc_deployer,
    )?;

    // ready to test!

    Ok(())
}

#[rstest]
#[case("utest")]
#[tokio::test]
async fn test_native_bridged(#[case] denom: &str) -> eyre::Result<()> {
    let osmo_app = OsmosisTestApp::new();
    let osmo = cw::setup_env(
        &osmo_app,
        |app, coins| app.init_account(coins).unwrap(),
        Some("../artifacts/"),
        "osmo",
        DOMAIN_OSMO,
        &[TestValidators::new(DOMAIN_EVM, 5, 3)],
        &[RemoteGasDataConfig {
            remote_domain: DOMAIN_EVM,
            token_exchange_rate: Uint128::from(10u128.pow(4)),
            gas_price: Uint128::from(10u128.pow(9)),
        }],
    )?;

    let wasm = Wasm::new(&osmo_app);
    let tf = TokenFactory::new(&osmo_app);

    let warp_resp = cw::deploy_warp_route_bridged(
        &wasm,
        &osmo.acc_owner,
        &osmo.acc_deployer,
        &osmo.core.mailbox,
        "osmo",
        &osmo.codes,
        denom.into(),
        warp::TokenType::Native(warp::TokenTypeNative::Fungible { denom: "".into() }),
    );

    let events = wasm_events(warp_resp.events);
    assert_eq!(events["wasm-hpl_warp_native::instantiate"]["denom"], denom);

    let new_denom = &events["wasm-hpl_warp_native::reply-init"]["denom"];
    assert_eq!(
        new_denom.clone(),
        format!("factory/{}/{denom}", warp_resp.data.address)
    );

    let resp = tf.query_denom_authority_metadata(
        &tokenfactory::v1beta1::QueryDenomAuthorityMetadataRequest {
            denom: new_denom.clone(),
        },
    )?;
    assert_eq!(
        resp.authority_metadata.unwrap().admin,
        warp_resp.data.address
    );

    Ok(())
}
