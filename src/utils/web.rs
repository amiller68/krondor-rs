use pulldown_cmark::{html, Options, Parser};

use crate::env::{APP_ITEMS_DIR, APP_MANIFEST_FILE};
use crate::error::{KrondorError, KrondorResult};
use crate::types::Manifest;

pub fn get_url() -> KrondorResult<String> {
    let url = web_sys::window().expect("window").origin();
    Ok(url)
}

pub fn get_item_url(name: &str) -> KrondorResult<String> {
    let url = web_sys::window().expect("window").origin();
    let url = format!("{}/{}/{}", url, APP_ITEMS_DIR, name);
    Ok(url)
}

pub fn get_manifest_url() -> KrondorResult<String> {
    let url = web_sys::window().expect("window").origin();
    let url = format!("{}/{}", url, APP_MANIFEST_FILE);
    Ok(url)
}

pub async fn get_manifest() -> KrondorResult<Manifest> {
    let url = get_manifest_url()?;
    let manifest = reqwest::get(url)
        .await
        .map_err(KrondorError::Request)?
        .json::<serde_json::Value>()
        .await
        .map_err(KrondorError::Request)?;
    let manifest: Manifest =
        serde_json::from_value::<Manifest>(manifest).map_err(KrondorError::Serde)?;

    Ok(manifest)
}

pub async fn get_item_text(name: &str) -> KrondorResult<String> {
    let url = get_item_url(name)?;
    reqwest::get(url)
        .await
        .map_err(KrondorError::Request)?
        .text()
        .await
        .map_err(KrondorError::Request)
}

pub fn fix_src(content: String) -> String {
    let url = get_url().expect("url");
    let content = content.replace("src=\"./", &format!("src=\"{}/{}/", url, APP_ITEMS_DIR));
    content
}

pub fn markdown_to_html(content: String) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(&content, options);
    let mut html = String::new();
    html::push_html(&mut html, parser);
    html
}
