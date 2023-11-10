use std::path::PathBuf;

use crate::env::{APP_DIST_DIR, APP_ITEMS_DIR, APP_MANIFEST_FILE};
use crate::error::{KrondorError, KrondorResult};
use crate::types::Manifest;

pub fn dist_path() -> PathBuf {
    PathBuf::from(APP_DIST_DIR)
}

pub fn manifest_path() -> PathBuf {
    dist_path().join(APP_MANIFEST_FILE)
}

pub fn post_path(name: &str) -> PathBuf {
    dist_path().join(APP_ITEMS_DIR).join(name)
}

pub fn init_space() -> KrondorResult<()> {
    let dist_path = dist_path();
    let manifest_path = manifest_path();
    if manifest_path.exists() {
        return Err(KrondorError::InvalidRequest(
            "Path is already initialized".to_string(),
        ));
    }
    std::fs::create_dir_all(dist_path).map_err(KrondorError::Io)?;
    let manifest = Manifest::default();
    manifest.save()?;
    Ok(())
}
