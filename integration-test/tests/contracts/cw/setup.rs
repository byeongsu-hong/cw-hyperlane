use std::{collections::BTreeMap, path::PathBuf};

use cosmwasm_std::{coin, Coin};
use test_tube::{Account, Module, Runner, SigningAccount, Wasm};

use crate::validator::TestValidators;

use super::{
    deploy_core, prepare_routing_hook, prepare_routing_ism, store_code, types::CoreDeployments,
};

const DEFAULT_GAS: u128 = 300_000;

pub struct Env<'a, R: Runner<'a>> {
    validators: BTreeMap<u32, TestValidators>,

    pub app: &'a R,
    pub core: CoreDeployments,
    pub domain: u32,

    acc_gen: Box<dyn Fn(&'a R, &'a [Coin]) -> SigningAccount>,
    pub acc_owner: SigningAccount,
    pub acc_tester: SigningAccount,
    pub acc_deployer: SigningAccount,
}

impl<'a, R: Runner<'a>> Env<'a, R> {
    pub fn get_validator_set(&self, domain: u32) -> eyre::Result<&TestValidators> {
        self.validators
            .get(&domain)
            .ok_or(eyre::eyre!("no validator set found"))
    }

    #[allow(dead_code)]
    pub fn gen_account(&'a self, coins: &'a [Coin]) -> SigningAccount {
        (self.acc_gen)(self.app, coins)
    }
}

pub fn setup_env<'a, R: Runner<'a>>(
    app: &'a R,
    acc_gen: impl Fn(&R, &[Coin]) -> SigningAccount + 'static,
    artifacts: Option<impl Into<PathBuf>>,
    hrp: &str,
    domain: u32,
    validators: &[TestValidators],
) -> eyre::Result<Env<'a, R>> {
    let owner = acc_gen(app, &[coin(1_000_000u128.pow(3), "uosmo")]);
    let deployer = acc_gen(app, &[coin(1_000_000u128.pow(3), "uosmo")]);
    let tester = acc_gen(app, &[coin(1_000_000u128.pow(3), "uosmo")]);

    let ism = prepare_routing_ism(
        validators
            .iter()
            .map(|v| (v.domain, hrp, v.clone()))
            .collect(),
    );
    let hook = prepare_routing_hook(
        deployer.address(),
        validators.iter().map(|v| (v.domain, DEFAULT_GAS)).collect(),
    );

    let wasm = Wasm::new(app);
    let codes = store_code(&wasm, &deployer, artifacts)?;
    let core = deploy_core(&wasm, &deployer, &codes, domain, hrp, ism, hook)?;

    Ok(Env {
        validators: validators.iter().map(|v| (v.domain, v.clone())).collect(),

        app,
        core,
        domain,

        acc_gen: Box::new(acc_gen),
        acc_owner: owner,
        acc_tester: tester,
        acc_deployer: deployer,
    })
}
