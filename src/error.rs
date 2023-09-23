use anyhow::Error;

pub type KrondorError = Error;
pub type KrondorResult<T> = std::result::Result<T, KrondorError>;
