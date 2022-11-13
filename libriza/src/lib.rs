mod browser;
mod compose;
mod error;
mod job;
mod parallelize;

pub use browser::{using_browser, RizaBrowserConfig, RizaDriverConfig};
pub use compose::compose;
pub use error::{RizaError, RizaResult};
pub use job::{run_workflow, RizaJob};
pub use parallelize::parallelize;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
