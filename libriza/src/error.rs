use thirtyfour::error::WebDriverError;
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum RizaError {
    #[error("web driver error")]
    WebDriverError(String),
    #[error("unknown error")]
    UnknownError(String),
}

pub type RizaResult<T> = Result<T, RizaError>;

impl From<WebDriverError> for RizaError {
    fn from(err: WebDriverError) -> Self {
        RizaError::WebDriverError(format!("{:?}", err))
    }
}
