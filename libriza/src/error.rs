use thiserror::Error;

#[derive(Debug, Error)]
pub enum RizaError {
    #[error("unknown error")]
    UnknownError(String),
}

pub type RizaResult<T> = Result<T, RizaError>;
