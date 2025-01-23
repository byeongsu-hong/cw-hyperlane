use cosmwasm_std::{Empty, HexBinary, StdResult};
use hpl_interface::types::pub_to_addr;
use osmosis_test_tube::Wasm;
use test_tube::{Account, Runner, SigningAccount};

use crate::validator::{self, TestValidators};

use super::types::Codes;

#[derive(Clone)]
pub enum Ism {
    Routing(Vec<(u32, Self)>),

    Multisig {
        validators: validator::TestValidators,
    },

    Aggregate {
        isms: Vec<Self>,
        threshold: u8,
    },

    #[allow(dead_code)]
    Mock,
}

impl Ism {
    pub fn routing(isms: Vec<(u32, Self)>) -> Self {
        Self::Routing(isms)
    }

    pub fn multisig(validators: validator::TestValidators) -> Self {
        Self::Multisig { validators }
    }
}

impl Ism {
    fn deploy_mock<'a, R: Runner<'a>>(
        wasm: &Wasm<'a, R>,
        codes: &Codes,
        deployer: &SigningAccount,
    ) -> eyre::Result<String> {
        Ok(wasm
            .instantiate(codes.test_mock_ism, &Empty {}, None, None, &[], deployer)?
            .data
            .address)
    }

    fn deploy_multisig<'a, R: Runner<'a>>(
        wasm: &Wasm<'a, R>,
        codes: &Codes,
        set: validator::TestValidators,
        owner: &SigningAccount,
        deployer: &SigningAccount,
    ) -> eyre::Result<String> {
        let multisig_ism = wasm
            .instantiate(
                codes.ism_multisig,
                &hpl_interface::ism::multisig::InstantiateMsg {
                    owner: owner.address(),
                },
                None,
                Some("None"),
                &[],
                deployer,
            )?
            .data
            .address;

        wasm.execute(
            &multisig_ism,
            &hpl_interface::ism::multisig::ExecuteMsg::SetValidators {
                domain: set.domain,
                threshold: set.threshold,
                validators: set
                    .validators
                    .iter()
                    .map(|v| {
                        pub_to_addr(HexBinary::from(
                            v.pub_key.to_encoded_point(true).as_bytes().to_vec(),
                        ))
                    })
                    .collect::<StdResult<Vec<_>>>()?,
            },
            &[],
            owner,
        )?;

        Ok(multisig_ism)
    }

    fn deploy_routing<'a, R: Runner<'a>>(
        wasm: &Wasm<'a, R>,
        codes: &Codes,
        isms: Vec<(u32, Self)>,
        owner: &SigningAccount,
        deployer: &SigningAccount,
    ) -> eyre::Result<String> {
        let routing_ism = wasm
            .instantiate(
                codes.ism_routing,
                &hpl_interface::ism::routing::InstantiateMsg {
                    owner: owner.address(),
                    isms: isms
                        .into_iter()
                        .map(|(domain, ism)| {
                            Ok(hpl_interface::ism::routing::IsmSet {
                                domain,
                                address: ism.deploy(wasm, codes, owner, deployer)?,
                            })
                        })
                        .collect::<eyre::Result<Vec<_>>>()?,
                },
                None,
                Some("None"),
                &[],
                deployer,
            )?
            .data
            .address;

        Ok(routing_ism)
    }

    fn deploy_aggregate<'a, R: Runner<'a>>(
        wasm: &Wasm<'a, R>,
        codes: &Codes,
        isms: Vec<Self>,
        threshold: u8,
        owner: &SigningAccount,
        deployer: &SigningAccount,
    ) -> eyre::Result<String> {
        use hpl_interface::ism::aggregate::*;

        let ism_addrs = isms
            .into_iter()
            .map(|v| v.deploy(wasm, codes, owner, deployer))
            .collect::<eyre::Result<Vec<_>>>()?;

        let aggregate_ism = wasm
            .instantiate(
                codes.ism_aggregate,
                &InstantiateMsg {
                    owner: owner.address(),
                    isms: ism_addrs,
                    threshold,
                },
                None,
                Some("None"),
                &[],
                deployer,
            )?
            .data
            .address;

        Ok(aggregate_ism)
    }

    pub fn deploy<'a, R: Runner<'a>>(
        self,
        wasm: &Wasm<'a, R>,
        codes: &Codes,
        owner: &SigningAccount,
        deployer: &SigningAccount,
    ) -> eyre::Result<String> {
        match self {
            Self::Mock => Self::deploy_mock(wasm, codes, deployer),
            Self::Multisig { validators: set } => {
                Self::deploy_multisig(wasm, codes, set, owner, deployer)
            }
            Self::Aggregate { isms, threshold } => {
                Self::deploy_aggregate(wasm, codes, isms, threshold, owner, deployer)
            }
            Self::Routing(isms) => Self::deploy_routing(wasm, codes, isms, owner, deployer),
        }
    }
}

pub fn prepare_routing_ism(info: Vec<(u32, TestValidators)>) -> Ism {
    let mut isms = vec![];

    for (domain, set) in info {
        isms.push((
            domain,
            Ism::Aggregate {
                isms: vec![Ism::multisig(set)],
                threshold: 1,
            },
        ));
    }

    Ism::routing(isms)
}
