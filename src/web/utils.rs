use serde::{Deserialize, Serialize};

use crate::error::{KrondorError, KrondorResult};
use crate::ipfs::GatewayClient;
use crate::types::{Cid, History, Item, Manifest};
use crate::env::{APP_METADATA_DIR, APP_MANIFEST_FILE, APP_COLLECTIONS_DIR};

pub fn get_url() -> KrondorResult<String> {
    let url = web_sys::window()
        .expect("window")
        .origin();
    Ok(url)
}

pub fn get_item_url(collection: &str, name: &str) -> KrondorResult<String> {
    let url = web_sys::window()
        .expect("window")
        .origin();
    let url = format!("{}/{}/{}/{}", url, APP_COLLECTIONS_DIR, collection, name); 
    Ok(url)
}

pub async fn get_manifest() -> KrondorResult<Manifest> {
    let url = web_sys::window()
        .expect("window")
        .location()
        .href()
        .expect("href");
    let url = format!("{}/{}/{}", url, APP_METADATA_DIR, APP_MANIFEST_FILE);
    let manifest = reqwest::get(url)
        .await
        .map_err(KrondorError::default)?
        .json::<serde_json::Value>()
        .await
        .map_err(KrondorError::default)?;
    let manifest: Manifest = serde_json::from_value::<Manifest>(manifest)
        .map_err(KrondorError::default)?;

    Ok(manifest)
}

pub async fn get_item_text(collection: &str, name: &str) -> KrondorResult<String> {
    let url = web_sys::window()
        .expect("window")
        .location()
        .href()
        .expect("href");
    // Strip off the cid from the url
    let url = url.split('/').take(3).collect::<Vec<&str>>().join("/");
    let url = format!("{}/{}/{}/{}", url, APP_COLLECTIONS_DIR, collection, name);
    reqwest::get(url)
        .await
        .map_err(KrondorError::default)?
        .text()
        .await
        .map_err(KrondorError::default)
}