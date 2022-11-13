use async_trait::async_trait;
use libriza::{
    compose, parallelize, run_workflow, using_browser, RizaBrowserConfig, RizaDriverConfig,
    RizaError, RizaJob, RizaResult,
};
use num::traits::CheckedAdd;
use std::marker::PhantomData;

#[tokio::main]
async fn main() {
    let wikipedia_seed = Box::new(Seed {
        config: PhantomData,
        data: "https://wikipedia.org".to_string(),
    });
    let wikipedia = Box::new(BrowserTitle {
        config: PhantomData,
        data: PhantomData,
    });
    let google_seed = Box::new(Seed {
        config: PhantomData,
        data: "https://google.ca".to_string(),
    });
    let google = Box::new(BrowserTitle {
        config: PhantomData,
        data: PhantomData,
    });
    let echo = Box::new(Echo {
        config: PhantomData,
        data: PhantomData,
    });
    let wikipedia_workflow = compose(wikipedia_seed, wikipedia);
    let google_workflow = compose(google_seed, google);
    let workflow = parallelize(vec![wikipedia_workflow, google_workflow]);
    let workflow = compose(workflow, echo);

    let config = Config {
        driver_config: RizaDriverConfig {
            server_url: "http://localhost:9515",
            headless: true,
        },
    };

    println!("{:?}", run_workflow(workflow, config).await);
}

struct Config<'a> {
    driver_config: RizaDriverConfig<'a>,
}

impl<'a> RizaBrowserConfig for Config<'a> {
    fn driver_config(&self) -> &RizaDriverConfig {
        &self.driver_config
    }
}

struct BrowserTitle<C, T> {
    config: PhantomData<C>,
    data: PhantomData<T>,
}

#[async_trait]
impl<C> RizaJob<C> for BrowserTitle<C, ()>
where
    C: RizaBrowserConfig + Send + Sync,
{
    type Input = String;
    type Output = String;

    async fn run(&self, config: &C, input: &Self::Input) -> RizaResult<Self::Output> {
        using_browser(config, |driver| async move {
            driver.goto(&input).await?;
            Ok(driver.title().await?)
        })
        .await
    }
}

struct Seed<C, T> {
    config: PhantomData<C>,
    data: T,
}

#[async_trait]
impl<C, T> RizaJob<C> for Seed<C, T>
where
    C: Send + Sync,
    T: Clone + Send + Sync,
{
    type Input = ();
    type Output = T;

    async fn run(&self, _config: &C, _input: &()) -> RizaResult<T> {
        Ok(self.data.clone())
    }
}

struct Echo<C, T> {
    config: PhantomData<C>,
    data: PhantomData<T>,
}

#[async_trait]
impl<C, T> RizaJob<C> for Echo<C, T>
where
    C: Send + Sync,
    T: Clone + Send + Sync,
{
    type Input = T;
    type Output = T;

    async fn run(&self, _config: &C, input: &T) -> RizaResult<T> {
        Ok(input.clone())
    }
}

struct Increment<C, T: CheckedAdd> {
    config: PhantomData<C>,
    by: T,
}

#[async_trait]
impl<C, T> RizaJob<C> for Increment<C, T>
where
    C: Send + Sync,
    T: CheckedAdd + Send + Sync,
{
    type Input = T;
    type Output = T;

    async fn run(&self, _config: &C, input: &T) -> RizaResult<T> {
        match input.checked_add(&self.by) {
            Some(value) => Ok(value),
            None => Err(RizaError::UnknownError("".to_string())),
        }
    }
}
