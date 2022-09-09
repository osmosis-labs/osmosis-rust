use crate::runner::Runner;

pub mod bank;
pub mod gamm;
pub mod wasm;

pub trait Module<'a, R: Runner> {
    fn new(runner: &'a R) -> Self;
}

pub trait AsModule<'a>: Runner + Sized {
    fn as_module<M>(&'a self) -> M
    where
        M: Module<'a, Self>,
    {
        M::new(self)
    }
}
