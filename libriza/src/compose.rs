use crate::job::RizaJob;
use async_trait::async_trait;

struct ComposedJob<C: Send + Sync, E, U: Send + Sync, V: Send + Sync, W: Send + Sync> {
    a: Box<dyn RizaJob<C, Error = E, Input = U, Output = V> + Send + Sync>,
    b: Box<dyn RizaJob<C, Error = E, Input = V, Output = W> + Send + Sync>,
}

#[async_trait]
impl<C, E, U, V, W> RizaJob<C> for ComposedJob<C, E, U, V, W>
where
    C: Send + Sync,
    U: Send + Sync,
    V: Send + Sync,
    W: Send + Sync,
{
    type Error = E;
    type Input = U;
    type Output = W;

    async fn run(&self, config: &C, input: &U) -> Result<Self::Output, Self::Error> {
        let input = self.a.run(config, input).await?;
        self.b.run(config, &input).await
    }
}

pub fn compose<
    C: Send + Sync + 'static,
    E: 'static,
    U: Send + Sync + 'static,
    V: Send + Sync + 'static,
    W: Send + Sync + 'static,
>(
    a: Box<dyn RizaJob<C, Error = E, Input = U, Output = V> + Send + Sync + 'static>,
    b: Box<dyn RizaJob<C, Error = E, Input = V, Output = W> + Send + Sync + 'static>,
) -> Box<dyn RizaJob<C, Error = E, Input = U, Output = W> + Send + Sync + 'static> {
    Box::new(ComposedJob { a, b })
}
