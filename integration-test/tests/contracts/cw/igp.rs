use cosmwasm_std::Uint128;
use hpl_interface::{
    igp::{self, oracle::RemoteGasDataConfig},
    router::{DomainRouteSet, RouterMsg},
};
use ibcx_test_utils::addr;
use test_tube::{Account, Runner, SigningAccount, Wasm};

use super::types::Codes;

#[derive(Clone)]
pub struct Igp {
    pub hrp: String,
    pub gas_token: String,
    pub beneficiary: String,
    pub oracle_configs: Vec<RemoteGasDataConfig>,
}

pub struct IgpDeployment {
    pub core: String,
    pub oracle: String,
}

impl Igp {
    pub fn deploy<'a, R: Runner<'a>>(
        self,
        wasm: &Wasm<'a, R>,
        codes: &Codes,
        owner: &SigningAccount,
        deployer: &SigningAccount,
    ) -> eyre::Result<IgpDeployment> {
        let igp = wasm
            .instantiate(
                codes.igp,
                &igp::core::InstantiateMsg {
                    hrp: self.hrp,
                    owner: owner.address(),
                    gas_token: self.gas_token,
                    beneficiary: self.beneficiary,
                    default_gas_usage: Uint128::from(25_000u128),
                },
                Some(deployer.address().as_str()),
                Some("cw-hpl-igp"),
                &[],
                deployer,
            )?
            .data
            .address;

        let igp_oracle = wasm
            .instantiate(
                codes.igp_oracle,
                &igp::oracle::InstantiateMsg {
                    owner: owner.address(),
                },
                Some(deployer.address().as_str()),
                Some("cw-hpl-igp-oracle"),
                &[],
                deployer,
            )?
            .data
            .address;

        wasm.execute(
            &igp,
            &igp::core::ExecuteMsg::Router(RouterMsg::SetRoutes {
                set: self
                    .oracle_configs
                    .iter()
                    .map(|v| DomainRouteSet {
                        domain: v.remote_domain,
                        route: Some(addr(&igp_oracle)),
                    })
                    .collect(),
            }),
            &[],
            owner,
        )?;

        if !self.oracle_configs.is_empty() {
            wasm.execute(
                &igp_oracle,
                &igp::oracle::ExecuteMsg::SetRemoteGasDataConfigs {
                    configs: self.oracle_configs,
                },
                &[],
                owner,
            )?;
        }

        Ok(IgpDeployment {
            core: igp,
            oracle: igp_oracle,
        })
    }
}
