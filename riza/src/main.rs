use async_trait::async_trait;
use libriza::{
    compose, using_browser, RizaBrowserConfig, RizaDriverConfig, RizaError, RizaJob, RizaResult,
};
use num::traits::CheckedAdd;
use std::marker::PhantomData;
use thirtyfour::By;

#[tokio::main]
async fn main() {
    let wikipedia = Box::new(BrowserVisitor {
        config: PhantomData,
        data: PhantomData,
    });
    let seed = Box::new(Seed {
        config: PhantomData,
        data: 100,
    });
    let echo = Box::new(Echo {
        config: PhantomData,
        data: PhantomData,
    });
    let inc1 = Box::new(Increment {
        config: PhantomData,
        by: 1,
    });
    let inc2 = Box::new(Increment {
        config: PhantomData,
        by: 2,
    });
    let workflow = compose(wikipedia, seed);
    let workflow = compose(workflow, echo);
    let workflow = compose(workflow, inc1);
    let workflow = compose(workflow, inc2);

    let config = Config {
        driver_config: RizaDriverConfig {
            server_url: "http://localhost:9515",
            headless: true,
        },
    };
    let data = ();
    println!("{:?}", workflow.run(&config, &data).await);
}

struct Config<'a> {
    driver_config: RizaDriverConfig<'a>,
}

impl<'a> RizaBrowserConfig for Config<'a> {
    fn driver_config(&self) -> &RizaDriverConfig {
        &self.driver_config
    }
}

struct BrowserVisitor<C, T> {
    config: PhantomData<C>,
    data: PhantomData<T>,
}

#[async_trait]
impl<C> RizaJob<C> for BrowserVisitor<C, ()>
where
    C: RizaBrowserConfig + Send + Sync,
{
    type Input = ();
    type Output = ();

    async fn run(&self, config: &C, _input: &()) -> RizaResult<()> {
        using_browser(config, |driver| async move {
            // Navigate to https://wikipedia.org.
            driver.goto("https://wikipedia.org").await?;

            let elem_form = driver.find(By::Id("search-form")).await?;

            // Find element from element.
            let elem_text = elem_form.find(By::Id("searchInput")).await?;

            // Type in the search terms.
            elem_text.send_keys("selenium").await?;

            // Click the search button.
            let elem_button = elem_form.find(By::Css("button[type='submit']")).await?;
            elem_button.click().await?;

            // Look for header to implicitly wait for the page to load.
            driver.find(By::ClassName("firstHeading")).await?;
            assert_eq!(driver.title().await?, "Selenium - Wikipedia");

            Ok(())
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
