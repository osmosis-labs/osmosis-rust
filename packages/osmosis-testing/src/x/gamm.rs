use cosmwasm_std::Coin;
use osmosis_std::types::osmosis::gamm::poolmodels::balancer::v1beta1::MsgCreateBalancerPoolResponse;
use osmosis_std::types::osmosis::gamm::{
    poolmodels::balancer::v1beta1::MsgCreateBalancerPool,
    v1beta1::{Pool, PoolAsset, PoolParams, QueryPoolRequest, QueryPoolResponse},
};
use prost::Message;

use crate::runner::result::RunnerExecuteResult;
use crate::x::Module;
use crate::{
    account::{Account, SigningAccount},
    runner::Runner,
};

pub struct Gamm<'a, R: Runner> {
    runner: &'a R,
}

impl<'a, R: Runner> Module<'a, R> for Gamm<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Gamm<'a, R>
where
    R: Runner,
{
    pub fn create_basic_pool(
        &self,
        initial_liquidity: &[Coin],
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgCreateBalancerPoolResponse> {
        self.runner.execute(
            MsgCreateBalancerPool {
                sender: signer.address(),
                pool_params: Some(PoolParams {
                    swap_fee: "10000000000000000".to_string(),
                    exit_fee: "10000000000000000".to_string(),
                    smooth_weight_change_params: None,
                }),
                pool_assets: initial_liquidity
                    .iter()
                    .map(|c| PoolAsset {
                        token: Some(osmosis_std::types::cosmos::base::v1beta1::Coin {
                            denom: c.denom.to_owned(),
                            amount: format!("{}", c.amount),
                        }),
                        weight: "1000000".to_string(),
                    })
                    .collect(),
                future_pool_governor: "".to_string(),
            },
            MsgCreateBalancerPool::TYPE_URL,
            signer,
        )
    }

    pub fn query_pool(&self, pool_id: u64) -> Pool {
        let any = self.runner.query::<QueryPoolRequest, QueryPoolResponse>(
            "/osmosis.gamm.v1beta1.Query/Pool",
            &QueryPoolRequest { pool_id },
        );

        Pool::decode(any.pool.unwrap().value.as_slice()).unwrap()
    }
}
