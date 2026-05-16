use thiserror::Error;

pub type Result<T> = std::result::Result<T, FlowfullError>;

#[derive(Debug, Error)]
pub enum FlowfullError {
    #[error("invalid configuration: {0}")]
    Config(String),

    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("api error: status={status}, message={message}")]
    Api {
        status: u16,
        message: String,
        body: Option<String>,
    },

    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("session error: {0}")]
    Session(String),

    #[error("storage error: {0}")]
    Storage(String),

    #[error("url error: {0}")]
    Url(#[from] url::ParseError),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("request failed after retries")]
    RequestFailed,
}
