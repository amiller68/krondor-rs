use anyhow::Error;

pub type WebError = Error;
pub type WebResult<T> = std::result::Result<T, WebError>;