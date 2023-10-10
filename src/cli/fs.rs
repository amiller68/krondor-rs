use std::path::PathBuf;

use crate::error::{KrondorError, KrondorResult};
use crate::types::{History, Manifest};
use crate::env::{APP_DIST_PATH, APP_METADATA_DIR, APP_MANIFEST_FILE, APP_HISTORY_FILE, APP_COLLECTIONS_DIR};
use crate::types::{POST_COLLECTION_DIR, GALLERY_COLLECTION_DIR};

pub fn dist_path() -> &'static str {
    APP_DIST_PATH
}

pub fn manifest_path() -> PathBuf  {
    let string = format!("{}/{}/{}", APP_DIST_PATH, APP_METADATA_DIR, APP_MANIFEST_FILE);
    PathBuf::from(string)
}

pub fn history_path() -> PathBuf {
    let string = format!("{}/{}/{}", APP_DIST_PATH, APP_METADATA_DIR, APP_HISTORY_FILE);
    PathBuf::from(string)
}

pub fn post_path(name: &str) -> PathBuf {
    let string = format!("{}/{}/{}/{}", APP_DIST_PATH, APP_COLLECTIONS_DIR, POST_COLLECTION_DIR, name);
    PathBuf::from(string)
}

pub fn gallery_path(name: &str) -> PathBuf {
    let string = format!("{}/{}/{}/{}", APP_DIST_PATH, APP_COLLECTIONS_DIR, GALLERY_COLLECTION_DIR, name);
    PathBuf::from(string)
}

pub fn init_space() -> KrondorResult<()> {
    let dist_path = std::path::Path::new(APP_DIST_PATH); 
    let metadata_path = dist_path.join(APP_METADATA_DIR);
    let manifest_path = metadata_path.join(APP_MANIFEST_FILE);
    let history_path = metadata_path.join(APP_HISTORY_FILE);

    let collections_path = dist_path.join(APP_COLLECTIONS_DIR);
    // TODO: This should be part of generated code
    let posts_path = collections_path.join(POST_COLLECTION_DIR);
    let gallery_path = collections_path.join(GALLERY_COLLECTION_DIR);

    if posts_path.exists() {
        return Err(KrondorError::msg("Posts directory already exists"));
    }
    if gallery_path.exists() {
        return Err(KrondorError::msg("Gallery directory already exists"));
    }
    if manifest_path.exists() {
        return Err(KrondorError::msg("Manifest file already exists"));
    }
    if history_path.exists() {
        return Err(KrondorError::msg("History file already exists"));
    }

    std::fs::create_dir_all(&dist_path).map_err(KrondorError::default)?;
    std::fs::create_dir_all(&metadata_path).map_err(KrondorError::default)?;
    std::fs::create_dir_all(&posts_path).map_err(KrondorError::default)?;
    std::fs::create_dir_all(&gallery_path).map_err(KrondorError::default)?;

    let manifest = Manifest::new();
    let history = History::new();

    manifest.save(&manifest_path)?;
    history.save(&history_path)?;

    Ok(())
}

        