use crate::runner::Runner;

pub mod gamm;
pub mod wasm;

pub trait Module<'a, R: Runner> {
    fn new(runner: &'a R) -> Self;
}
pub trait AsModule<'a, M, R>
where
    M: Module<'a, R>,
    R: Runner,
{
    fn as_module(&'a self) -> M;
}
