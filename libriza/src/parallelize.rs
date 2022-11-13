use crate::error::RizaResult;
use crate::job::RizaJob;
use async_trait::async_trait;

struct ParallelizedJob<C: Send + Sync, U: Send + Sync, V: Send + Sync> {
    jobs: Vec<Box<dyn RizaJob<C, Input = U, Output = V> + Send + Sync>>,
}

#[async_trait]
impl<C, U, V> RizaJob<C> for ParallelizedJob<C, U, V>
where
    C: Send + Sync,
    U: Send + Sync,
    V: Send + Sync,
{
    type Input = U;
    type Output = Vec<V>;

    async fn run(&self, config: &C, input: &U) -> RizaResult<Self::Output> {
        let mut inputs = vec![];

        for job in self.jobs.iter() {
            let input = job.run(config, input).await?;
            inputs.push(input);
        }

        Ok(inputs)
    }
}

pub fn parallelize<C: Send + Sync + 'static, U: Send + Sync + 'static, V: Send + Sync + 'static>(
    jobs: Vec<Box<dyn RizaJob<C, Input = U, Output = V> + Send + Sync + 'static>>,
) -> Box<dyn RizaJob<C, Input = U, Output = Vec<V>> + Send + Sync + 'static> {
    Box::new(ParallelizedJob { jobs })
}
