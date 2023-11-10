pub type KrondorResult<T> = std::result::Result<T, KrondorError>;

#[derive(Debug, thiserror::Error)]
pub enum KrondorError {
    // Disk I/O errors
    #[cfg(not(target_arch = "wasm32"))]
    #[error("Disk I/O error: {0}")]
    Io(#[from] std::io::Error),

    // Cid errors
    #[error("Cid error: {0}")]
    Cid(#[from] cid::Error),

    #[error("Multihash error: {0}")]
    Multihash(#[from] cid::multihash::Error),

    // Serde errors
    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Reqwest error: {0}")]
    Request(#[from] reqwest::Error),

    // User triggered errors
    #[error("Invalid Request: {0}")]
    InvalidRequest(String),
}
