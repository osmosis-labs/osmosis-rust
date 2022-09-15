use crate::runner::Runner;

pub mod bank;
pub mod gamm;
pub mod wasm;

pub trait Module<'a, R: Runner> {
    fn new(runner: &'a R) -> Self;
}
