use anyhow::Error;

pub type KrondorResult<T> = std::result::Result<T, KrondorError>;

pub struct KrondorError {
    kind: KrondorErrorKind,
}

impl KrondorError {
    pub fn default(err: impl Into<Error>) -> Self {
        Self {
            kind: KrondorErrorKind::Default(err.into()),
        }
    }
    pub fn msg(msg: &str) -> Self {
        Self {
            kind: KrondorErrorKind::Msg(msg.to_string()),
        }
    }
}

impl std::fmt::Debug for KrondorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            KrondorErrorKind::Default(err) => write!(f, "{:?}", err),
            KrondorErrorKind::Msg(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::fmt::Display for KrondorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            KrondorErrorKind::Default(err) => write!(f, "{}", err),
            KrondorErrorKind::Msg(msg) => write!(f, "{}", msg),
        }
    }
}

pub enum KrondorErrorKind {
    Default(Error),
    Msg(String),
}
