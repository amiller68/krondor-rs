use std::path::PathBuf;

use crate::env::{
    APP_MANIFEST_FILE, APP_POSTS_DIR, APP_DIST_DIR
};
use crate::error::{KrondorError, KrondorResult};
use crate::types::Manifest;

pub fn dist_path() -> PathBuf {
    PathBuf::from(APP_DIST_DIR)
}
pub fn manifest_path() -> PathBuf {
    dist_path().join(APP_MANIFEST_FILE)
}

pub fn post_path(name: &str) -> PathBuf {
    dist_path().join(APP_POSTS_DIR).join(name)
}

pub fn init_space() -> KrondorResult<()> {
    let dist_path = dist_path();
    let manifest_path = manifest_path();

    if manifest_path.exists() {
        return Err(KrondorError::msg("Manifest file already exists"));
    }

    std::fs::create_dir_all(&dist_path).map_err(KrondorError::default)?;

    let manifest = Manifest::new();

    manifest.save(&manifest_path)?;

    Ok(())
}
