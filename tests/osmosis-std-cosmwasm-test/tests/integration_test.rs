mod helpers;
use helpers::with_env_setup;
use osmosis_std::types::osmosis::epochs::v1beta1::EpochInfo;
use osmosis_std_cosmwasm_test::msg::{QueryEpochsInfoResponse, QueryMsg, QueryNumPoolsResponse};

#[test]
fn test_u64_response_deser() {
    with_env_setup(|_app, wasm, _signer, _code_id, contract_addr| {
        assert_eq!(
            QueryNumPoolsResponse { num_pools: 0 },
            wasm.query(&contract_addr, &QueryMsg::QueryNumPools {})
                .unwrap()
        )
    })
}

#[test]
fn test_bool_response_deser() {
    with_env_setup(|_app, wasm, _signer, _code_id, contract_addr| {
        let res: QueryEpochsInfoResponse = wasm
            .query(&contract_addr, &QueryMsg::QueryEpochsInfo {})
            .unwrap();
        let day = &res.epochs[0];

        let EpochInfo {
            identifier,
            start_time: _,
            duration: _,
            current_epoch,
            current_epoch_start_time: _,
            epoch_counting_started,
            current_epoch_start_height,
        } = day;

        assert_eq!(identifier, "day");
        assert_eq!(current_epoch, &4);
        assert_eq!(epoch_counting_started, &true);
        assert_eq!(current_epoch_start_height, &4);
    })
}
