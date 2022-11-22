use crate::{fn_execute, fn_query};
use cosmrs::proto::cosmos::bank::v1beta1::{
    MsgSend, MsgSendResponse, QueryAllBalancesRequest, QueryAllBalancesResponse,
    QueryBalanceRequest, QueryBalanceResponse, QueryTotalSupplyRequest, QueryTotalSupplyResponse,
};

use crate::module::Module;
use crate::runner::Runner;

pub struct Bank<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Bank<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Bank<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! {
        pub send: MsgSend["/cosmos.bank.v1beta1.Msg/Send"] => MsgSendResponse
    }

    fn_query! {
        pub query_balance ["/cosmos.bank.v1beta1.Query/Balance"]: QueryBalanceRequest => QueryBalanceResponse
    }

    fn_query! {
        pub query_all_balances ["/cosmos.bank.v1beta1.Query/AllBalances"]: QueryAllBalancesRequest => QueryAllBalancesResponse
    }

    fn_query! {
        pub query_total_supply ["/cosmos.bank.v1beta1.Query/TotalSupply"]: QueryTotalSupplyRequest => QueryTotalSupplyResponse
    }
}
