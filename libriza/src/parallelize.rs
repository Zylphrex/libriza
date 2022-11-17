use crate::job::RizaJob;
use async_trait::async_trait;

struct ParallelizedJob<C: Send + Sync, E, U: Send + Sync, V: Send + Sync> {
    jobs: Vec<Box<dyn RizaJob<C, Error = E, Input = U, Output = V> + Send + Sync>>,
}

#[async_trait]
impl<C, E, U, V> RizaJob<C> for ParallelizedJob<C, E, U, V>
where
    C: Send + Sync,
    U: Send + Sync,
    V: Send + Sync,
{
    type Error = E;
    type Input = U;
    type Output = Vec<V>;

    async fn run(&self, config: &C, input: &U) -> Result<Self::Output, Self::Error> {
        let mut inputs = vec![];

        for job in self.jobs.iter() {
            let input = job.run(config, input).await?;
            inputs.push(input);
        }

        Ok(inputs)
    }
}

pub fn parallelize<
    C: Send + Sync + 'static,
    E: 'static,
    U: Send + Sync + 'static,
    V: Send + Sync + 'static,
>(
    jobs: Vec<Box<dyn RizaJob<C, Error = E, Input = U, Output = V> + Send + Sync + 'static>>,
) -> Box<dyn RizaJob<C, Error = E, Input = U, Output = Vec<V>> + Send + Sync + 'static> {
    Box::new(ParallelizedJob { jobs })
}
