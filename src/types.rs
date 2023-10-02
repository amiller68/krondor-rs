
use std::convert::{From, TryFrom};
use anyhow::Result;
use std::path::Path;

use cid::Cid as BaseCid;
use leptos::{IntoView, View};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(serde::Deserialize, Serialize)]
pub struct Config {
    pub(crate) version: String,
    pub(crate) path: String,
    pub(crate) ipfs_endpoint: String,
    pub(crate) ipfs_gateway: String,
}

#[cfg(not(target_arch = "wasm32"))]
impl Config {
    pub fn new() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            path: "./public".to_string(),
            ipfs_endpoint: "http://localhost:5001".to_string(),
            ipfs_gateway: "http://localhost:8080".to_string(),
        }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load(path: &Path) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let config = serde_json::from_str::<Self>(&json)?;
        Ok(config)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Manifest {
    pub version: String,
    pub posts: Vec<Post>,
    pub gallery: Vec<GalleryItem>,
}

#[cfg(not(target_arch = "wasm32"))]
impl Manifest {
    pub fn new() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            posts: Vec::new(),
            gallery: Vec::new(),
        }
    }
    pub fn load(path: &Path) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let manifest = serde_json::from_str::<Self>(&json)?;
        Ok(manifest)
    }
    pub fn save(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct History {
    pub previous: Cid,
}

#[cfg(not(target_arch = "wasm32"))]
impl History {
    pub fn new() -> Self {
        Self {
            previous: Cid::default(),
        }
    }
    pub fn load(path: &Path) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let history = serde_json::from_str::<Self>(&json)?;
        Ok(history)
    }
    pub fn save(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cid(BaseCid);

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Post {
    cid: Cid,
    name: String,
    title: String,
    date: String,
}

impl Post {
    pub fn cid(&self) -> Cid {
        self.cid
    }
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn title(&self) -> &str {
        self.title.as_str()
    }
    pub fn date(&self) -> &str {
        self.date.as_str()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct GalleryItem {
    cid: Cid,
    name: String,
    date: String,
}

impl GalleryItem {
    pub fn cid(&self) -> Cid {
        self.cid
    }
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn date(&self) -> &str {
        self.date.as_str()
    }
}

impl Cid {
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<Cid> for BaseCid {
    fn from(cid: Cid) -> Self {
        cid.0
    }
}

impl IntoView for Cid {
    fn into_view(self, cx: leptos::Scope) -> View {
        use leptos::leptos_dom::Text;
        let text = self.to_string();
        let text = Text::new(text.into());
        text.into_view(cx)
    }
}

impl Serialize for Cid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'de> Deserialize<'de> for Cid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Cid(BaseCid::try_from(s).unwrap()))
    }
}