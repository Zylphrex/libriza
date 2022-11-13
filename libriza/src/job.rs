use crate::error::RizaResult;
use async_trait::async_trait;

#[async_trait]
pub trait RizaJob<C> {
    type Input;
    type Output;

    async fn run(&self, config: &C, input: &Self::Input) -> RizaResult<Self::Output>;
}

pub async fn run_workflow<C: Send + Sync, T: Send + Sync>(
    job: Box<dyn RizaJob<C, Input = (), Output = T>>,
    config: C,
) -> RizaResult<T> {
    job.run(&config, &()).await
}
