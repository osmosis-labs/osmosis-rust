use crate::runner::result::{RunnerExecuteResult, RunnerResult};

use crate::account::SigningAccount;

pub mod app;
pub mod error;
pub mod result;

pub trait Runner {
    fn execute<M, R>(
        &self,
        msg: M,
        type_url: &str,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<R>
    where
        M: ::prost::Message,
        R: ::prost::Message + Default;
    fn query<Q, R>(&self, path: &str, query: &Q) -> RunnerResult<R>
    where
        Q: ::prost::Message,
        R: ::prost::Message + Default;
}
