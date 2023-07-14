use cosmwasm_std::{to_binary, Deps, HexBinary, QueryResponse};
use hpl_interface::mailbox::{
    CheckPointResponse, CountResponse, DefaultIsmResponse, MessageDeliveredResponse, NonceResponse,
    PausedResponse, RootResponse,
};

use crate::{
    state::{CONFIG, MESSAGE_PROCESSED, MESSAGE_TREE, NONCE, PAUSE},
    verify, ContractError,
};

pub fn get_delivered(deps: Deps, id: HexBinary) -> Result<QueryResponse, ContractError> {
    let delivered = MESSAGE_PROCESSED
        .load(deps.storage, id.into())
        .map_err(|_| ContractError::MessageNotFound {})?;

    Ok(to_binary(&MessageDeliveredResponse { delivered })?)
}

pub fn get_root(deps: Deps) -> Result<QueryResponse, ContractError> {
    let root = MESSAGE_TREE.load(deps.storage)?.root()?.into();

    Ok(to_binary(&RootResponse { root })?)
}

pub fn get_count(deps: Deps) -> Result<QueryResponse, ContractError> {
    let count = MESSAGE_TREE.load(deps.storage)?.count;

    Ok(to_binary(&CountResponse { count })?)
}

pub fn get_checkpoint(deps: Deps) -> Result<QueryResponse, ContractError> {
    let tree = MESSAGE_TREE.load(deps.storage)?;

    Ok(to_binary(&CheckPointResponse {
        root: tree.root()?.into(),
        count: tree.count,
    })?)
}

pub fn get_paused(deps: Deps) -> Result<QueryResponse, ContractError> {
    let paused = PAUSE.load(deps.storage)?;
    Ok(to_binary(&PausedResponse { paused })?)
}

pub fn get_nonce(deps: Deps) -> Result<QueryResponse, ContractError> {
    let nonce = NONCE.load(deps.storage)?;
    Ok(to_binary(&NonceResponse { nonce })?)
}

pub fn get_default_ism(deps: Deps) -> Result<QueryResponse, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    Ok(to_binary(&DefaultIsmResponse {
        default_ism: verify::bech32_decode(config.default_ism).into(),
    })?)
}

#[cfg(test)]
mod test {
    use crate::merkle::MerkleTree;

    use super::*;
    use cosmwasm_std::{testing::mock_dependencies, HexBinary, StdError};

    #[test]
    fn test_get_delivery() {
        let mut deps = mock_dependencies();
        let id = HexBinary::from_hex("c0ffee").unwrap();
        // cannot find deps delivery
        let notfound_resp = get_delivered(deps.as_ref(), id.clone()).unwrap_err();
        assert!(matches!(notfound_resp, ContractError::MessageNotFound {}));

        // set delivery
        MESSAGE_PROCESSED
            .save(deps.as_mut().storage, id.clone().into(), &true)
            .unwrap();

        let resp = get_delivered(deps.as_ref(), id).unwrap();
        assert_eq!(
            resp,
            to_binary(&MessageDeliveredResponse { delivered: true }).unwrap()
        );
    }
}
