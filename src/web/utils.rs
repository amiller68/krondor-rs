use crate::env::{APP_POSTS_DIR, APP_MANIFEST_FILE};
use crate::error::{KrondorError, KrondorResult};
use crate::types::Manifest;

pub fn get_url() -> KrondorResult<String> {
    let url = web_sys::window().expect("window").origin();
    Ok(url)
}

pub fn get_item_url(name: &str) -> KrondorResult<String> {
    let url = web_sys::window().expect("window").origin();
    let url = format!("{}/{}/{}", url, APP_POSTS_DIR, name);
    Ok(url)
}

pub async fn get_manifest() -> KrondorResult<Manifest> {
    let url = web_sys::window()
        .expect("window")
        .location()
        .origin()
        .expect("href");
    let url = format!("{}/{}", url, APP_MANIFEST_FILE);
    let manifest = reqwest::get(url)
        .await
        .map_err(KrondorError::default)?
        .json::<serde_json::Value>()
        .await
        .map_err(KrondorError::default)?;
    let manifest: Manifest =
        serde_json::from_value::<Manifest>(manifest).map_err(KrondorError::default)?;

    Ok(manifest)
}

pub async fn get_item_text(name: &str) -> KrondorResult<String> {
    gloo::console::log!("get_item_text: {}", name);
    let url = web_sys::window()
        .expect("window")
        .location()
        .href()
        .expect("href");
    // Strip off the cid from the url
    let url = url.split('/').take(3).collect::<Vec<&str>>().join("/");
    let url = format!("{}/{}/{}", url, APP_POSTS_DIR, name);
    reqwest::get(url)
        .await
        .map_err(KrondorError::default)?
        .text()
        .await
        .map_err(KrondorError::default)
}
