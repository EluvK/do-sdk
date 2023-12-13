use thiserror::Error;

#[derive(Debug, Error)]
pub enum DoError {
    #[error("http error: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("json error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("api error: {0}")]
    ApiRespError(String),
}

pub type DoResult<T> = Result<T, DoError>;
