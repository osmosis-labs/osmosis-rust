mod helpers;
use helpers::with_env_setup;
use osmosis_std::{shim::Timestamp, types::osmosis::epochs::v1beta1::EpochInfo};
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
            start_time,
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

#[test]
fn test_timestamp_response_deser() {
    with_env_setup(|_app, wasm, _signer, _code_id, contract_addr| {
        let res: QueryEpochsInfoResponse = wasm
            .query(&contract_addr, &QueryMsg::QueryEpochsInfo {})
            .unwrap();
        let day = &res.epochs[0];

        let EpochInfo {
            identifier: _,
            start_time,
            duration: _,
            current_epoch: _,
            current_epoch_start_time,
            epoch_counting_started: _,
            current_epoch_start_height: _,
        } = day;

        assert_eq!(
            // 0001-01-01T00:00:00Z
            start_time.as_ref().unwrap().to_owned(),
            Timestamp {
                seconds: -62135596800,
                nanos: 0
            }
        );

        assert_eq!(
            // 0001-01-04T00:00:00Z (+3 days from start_time)
            current_epoch_start_time.as_ref().unwrap().to_owned(),
            Timestamp {
                seconds: -62135596800 + (3 * 24 * 60 * 60),
                nanos: 0
            }
        );
    })
}
