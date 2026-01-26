use thiserror::Error;

#[derive(Error, Debug)]
pub enum FetchError {
    #[error("Unable to fetch data")]
    FetchError(#[from] rust_mc_status::McError),
    
    #[error("{0}")]
    General(String),
}

impl From<&str> for FetchError {
    fn from(s: &str) -> Self {
        FetchError::General(s.to_string())
    }
}

#[derive(Error, Debug)]
pub enum TelegramError {
    #[error("HTTP request failed")]
    RequestError(#[from] reqwest::Error),
    
    #[error("Telegram API error: {0}")]
    ApiError(String),
}
