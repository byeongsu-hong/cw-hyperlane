use cosmwasm_std::{to_binary, DepsMut, HexBinary, Response, WasmMsg};
use hpl_interface::types::message::Message;

use crate::{event::emit_post_dispatch, state::HOOK_CONFIG, ContractError};

pub fn dispatch(
    deps: DepsMut,
    metadata: HexBinary,
    message: HexBinary,
) -> Result<Response, ContractError> {
    let hpl_msg: Message = message.clone().into();
    let target_contract = HOOK_CONFIG
        .load(deps.storage, hpl_msg.dest_domain)
        .map_err(|_| ContractError::HookNotRegistered(hpl_msg.dest_domain))?;

    let wasm_msg = WasmMsg::Execute {
        contract_addr: target_contract.hook.to_string(),
        msg: to_binary(
            &hpl_interface::post_dispatch_hook::PostDispatchMsg::PostDispatch {
                metadata: metadata.clone(),
                message: message.clone(),
            },
        )?,
        funds: vec![],
    };

    Ok(Response::new()
        .add_message(wasm_msg)
        .add_event(emit_post_dispatch(target_contract.hook, metadata, message)))
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{testing::mock_dependencies, Addr};
    use hpl_interface::{hook::HookConfig, post_dispatch_hook::PostDispatchMsg};

    use super::*;
    const HOOK_ADDR: &str = "osmoaddress";

    #[test]
    fn test_dispatch() {
        let mut deps = mock_dependencies();

        // dest_domain: 11155111
        let binary_message = HexBinary::from_hex("00000021500000aef3000000000000000000000000477d860f8f41bc69ddd32821f2bf2c2af0243f1600aa36a70000000000000000000000005d56b8a669f50193b54319442c6eee5edd66238148656c6c6f21").unwrap();
        let dummy_metadata = HexBinary::from_hex("deadbeefc0ffee").unwrap();
        let hook = Addr::unchecked(HOOK_ADDR);

        let domain_not_exist = dispatch(
            deps.as_mut(),
            dummy_metadata.clone(),
            binary_message.clone(),
        )
        .unwrap_err();
        assert!(matches!(
            domain_not_exist,
            ContractError::HookNotRegistered(11155111)
        ));

        // success
        let hook_config = HookConfig {
            hook: hook.clone(),
            destination: 11155111,
        };
        HOOK_CONFIG
            .save(deps.as_mut().storage, 11155111, &hook_config)
            .unwrap();

        let expect_msg = WasmMsg::Execute {
            contract_addr: hook.to_string(),
            msg: to_binary(&PostDispatchMsg::PostDispatch {
                metadata: dummy_metadata.clone(),
                message: binary_message.clone(),
            })
            .unwrap(),
            funds: vec![],
        };

        let res = dispatch(
            deps.as_mut(),
            dummy_metadata.clone(),
            binary_message.clone(),
        )
        .unwrap();

        assert_eq!(
            res,
            Response::new()
                .add_message(expect_msg)
                .add_event(emit_post_dispatch(hook, dummy_metadata, binary_message))
        )
    }
}
