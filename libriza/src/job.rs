use async_trait::async_trait;
use crate::error::RizaResult;

#[async_trait]
pub trait RizaJob<C> {
    type Input;
    type Output;

    async fn run(&self, config: &C, input: &Self::Input) -> RizaResult<Self::Output>;
}
