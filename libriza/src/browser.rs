use std::future::Future;
use thirtyfour::prelude::*;

pub struct RizaDriverConfig<'a> {
    pub server_url: &'a str,
    pub headless: bool,
}

pub trait RizaBrowserConfig {
    fn driver_config(&self) -> &RizaDriverConfig;
}

pub async fn using_browser<C, E, T, F, Fut>(config: &C, func: F) -> Result<T, E>
where
    C: RizaBrowserConfig,
    E: From<WebDriverError>,
    F: FnOnce(WebDriver) -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    let driver_config = config.driver_config();
    let mut caps = DesiredCapabilities::chrome();
    if driver_config.headless {
        caps.set_headless()?;
    }

    let driver = WebDriver::new(driver_config.server_url, caps).await?;

    let result = func(driver.clone()).await;

    let _ = driver.quit().await?;

    result
}
