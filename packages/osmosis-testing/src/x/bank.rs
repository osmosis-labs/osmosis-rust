use crate::runner::result::RunnerResult;
use cosmrs::proto::cosmos::bank::v1beta1::{QueryAllBalancesRequest, QueryAllBalancesResponse};
use cosmrs::proto::cosmos::base::query::v1beta1::PageRequest;

use crate::runner::Runner;
use crate::x::Module;

pub struct Bank<'a, R: Runner> {
    runner: &'a R,
}

impl<'a, R: Runner> Module<'a, R> for Bank<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Bank<'a, R>
where
    R: Runner,
{
    pub fn query_all_balances(
        &self,
        address: &str,
        pagination: Option<PageRequest>,
    ) -> RunnerResult<QueryAllBalancesResponse> {
        self.runner
            .query::<QueryAllBalancesRequest, QueryAllBalancesResponse>(
                "/cosmos.bank.v1beta1.Query/AllBalances",
                &QueryAllBalancesRequest {
                    address: address.to_string(),
                    pagination,
                },
            )
    }
}
