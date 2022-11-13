use async_trait::async_trait;
use libriza::{compose, RizaResult, RizaError, RizaJob};
use num::traits::CheckedAdd;
use std::marker::PhantomData;
use thirtyfour::prelude::*;

#[tokio::main]
async fn main() {
    let noop = Box::new(Noop {
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
    let workflow = compose(noop, seed);
    let workflow = compose(workflow, echo);
    let workflow = compose(workflow, inc1);
    let workflow = compose(workflow, inc2);

    let config = Config {};
    let data = ();
    println!("{:?}", workflow.run(&config, &data).await);
}

struct Config {}

fn f<T>(a: Result<T, WebDriverError>) -> RizaResult<T> {
    match a {
        Ok(a) => Ok(a),
        Err(err) => Err(RizaError::UnknownError(format!("{:?}", err))),
    }
}

struct Noop<C, T> {
    config: PhantomData<C>,
    data: PhantomData<T>,
}

#[async_trait]
impl<C> RizaJob<C> for Noop<C, ()>
where
    C: Send + Sync,
{
    type Input = ();
    type Output = ();

    async fn run(&self, _config: &C, _input: &()) -> RizaResult<()> {
        let mut caps = DesiredCapabilities::chrome();
        f(caps.set_headless())?;
        let driver = f(WebDriver::new("http://localhost:9515", caps).await)?;

        // Navigate to https://wikipedia.org.
        f(driver.goto("https://wikipedia.org").await)?;
        let elem_form = f(driver.find(By::Id("search-form")).await)?;

        // Find element from element.
        let elem_text = f(elem_form.find(By::Id("searchInput")).await)?;

        // Type in the search terms.
        f(elem_text.send_keys("selenium").await)?;

        // Click the search button.
        let elem_button = f(elem_form.find(By::Css("button[type='submit']")).await)?;
        f(elem_button.click().await)?;

        // Look for header to implicitly wait for the page to load.
        f(driver.find(By::ClassName("firstHeading")).await)?;
        assert_eq!(f(driver.title().await)?, "Selenium - Wikipedia");

        // Always explicitly close the browser.
        f(driver.quit().await)?;
        Ok(())
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
