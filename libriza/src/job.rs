use async_trait::async_trait;

#[async_trait]
pub trait RizaJob<C> {
    type Error;
    type Input;
    type Output;

    async fn run(&self, config: &C, input: &Self::Input) -> Result<Self::Output, Self::Error>;
}

pub async fn run_workflow<C: Send + Sync, E, T: Send + Sync>(
    job: Box<dyn RizaJob<C, Input = (), Output = T, Error = E>>,
    config: C,
) -> Result<T, E> {
    job.run(&config, &()).await
}
