mod browser;
mod compose;
mod error;
mod job;

pub use browser::{using_browser, RizaBrowserConfig, RizaDriverConfig};
pub use compose::compose;
pub use error::{RizaError, RizaResult};
pub use job::RizaJob;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
