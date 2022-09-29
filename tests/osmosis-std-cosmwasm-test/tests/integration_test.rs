mod helpers;
use helpers::with_env_setup;
use osmosis_std::{
    shim::{Duration, Timestamp},
    types::osmosis::epochs::v1beta1::EpochInfo,
};
use osmosis_std_cosmwasm_test::msg::{
    QueryEpochsInfoResponse, QueryMsg, QueryNumPoolsResponse, QueryPoolParamsResponse,
    QueryPoolResponse,
};
use osmosis_testing::osmosis_std::types::osmosis::gamm::v1beta1::{Pool, PoolAsset, PoolParams};

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

#[test]
fn test_duration_response_deser() {
    with_env_setup(|_app, wasm, _signer, _code_id, contract_addr| {
        let res: QueryEpochsInfoResponse = wasm
            .query(&contract_addr, &QueryMsg::QueryEpochsInfo {})
            .unwrap();
        let day = &res.epochs[0];

        let EpochInfo {
            identifier: _,
            start_time: _,
            duration,
            current_epoch: _,
            current_epoch_start_time: _,
            epoch_counting_started: _,
            current_epoch_start_height: _,
        } = day;

        assert_eq!(
            duration.as_ref().unwrap().to_owned(),
            Duration {
                seconds: 86400,
                nanos: 0
            }
        );
    })
}

#[test]
fn test_any_balancer_pool_response_deser() {
    with_env_setup(|app, wasm, signer, _code_id, contract_addr| {
        let pools = helpers::setup_pools(app, &signer);
        let pool_id = pools[0];

        let res: QueryPoolResponse = wasm
            .query(&contract_addr, &QueryMsg::QueryPool { pool_id })
            .unwrap();

        let pool: Pool = res.pool.unwrap().try_into().unwrap();
        assert_eq!(
            pool,
            Pool {
                address: "osmo1mw0ac6rwlp5r8wapwk3zs6g29h8fcscxqakdzw9emkne6c8wjp9q0t3v8t"
                    .to_string(),
                id: 1,
                pool_params: Some(PoolParams {
                    swap_fee: "0.010000000000000000".to_string(),
                    exit_fee: "0.010000000000000000".to_string(),
                    smooth_weight_change_params: None,
                },),
                future_pool_governor: "".to_string(),
                total_shares: Some(osmosis_std::types::cosmos::base::v1beta1::Coin {
                    denom: "gamm/pool/1".to_string(),
                    amount: "100000000000000000000".to_string(),
                },),
                pool_assets: vec![
                    PoolAsset {
                        token: Some(osmosis_std::types::cosmos::base::v1beta1::Coin {
                            denom: "uion".to_string(),
                            amount: "1000".to_string(),
                        },),
                        weight: "1073741824000000".to_string(),
                    },
                    PoolAsset {
                        token: Some(osmosis_std::types::cosmos::base::v1beta1::Coin {
                            denom: "uosmo".to_string(),
                            amount: "1000".to_string(),
                        },),
                        weight: "1073741824000000".to_string(),
                    },
                ],
                total_weight: "2147483648000000".to_string(),
            }
        );
    });
}

#[test]
fn test_any_balancer_pool_params_response_deser() {
    with_env_setup(|app, wasm, signer, _code_id, contract_addr| {
        let pools = helpers::setup_pools(app, &signer);
        let pool_id = pools[0];

        let res: QueryPoolParamsResponse = wasm
            .query(&contract_addr, &QueryMsg::QueryPoolParams { pool_id })
            .unwrap();

        let pool: PoolParams = res.params.unwrap().try_into().unwrap();

        assert_eq!(
            pool,
            PoolParams {
                swap_fee: "0.010000000000000000".to_string(),
                exit_fee: "0.010000000000000000".to_string(),
                smooth_weight_change_params: None,
            }
        );
    });
}
// TODO: add tests for stableswap and query pools
