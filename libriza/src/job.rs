use crate::error::RizaResult;
use async_trait::async_trait;

#[async_trait]
pub trait RizaJob<C> {
    type Input;
    type Output;

    async fn run(&self, config: &C, input: &Self::Input) -> RizaResult<Self::Output>;
}
