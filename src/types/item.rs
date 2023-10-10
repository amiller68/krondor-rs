use std::convert::From;
use std::path::Path;

use crate::env;
use crate::error::{KrondorResult, KrondorError};

use super::Cid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

// TODO: This file should be generated from a config file somehow

const POST_COLLECTION: &'static str = "posts";
const GALLERY_COLLECTION: &'static str = "gallery";

pub const POST_COLLECTION_DIR: &'static str = "posts";
pub const GALLERY_COLLECTION_DIR: &'static str = "gallery";

lazy_static::lazy_static!(
    static ref POST_COLLECTION_PATH: String = format!(
        "{}/{}/{}",
        env::APP_DIST_PATH,
        env::APP_COLLECTIONS_DIR,
        POST_COLLECTION
    );
    static ref GALLERY_COLLECTION_PATH: String = format!(
        "{}/{}/{}",
        env::APP_DIST_PATH,
        env::APP_COLLECTIONS_DIR,
        GALLERY_COLLECTION
    );
);

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
/// A piece of data asscoated with content
pub struct InnerItem {
    /// The CID of the item
    cid: Cid,
    /// The name of the item
    name: String,
    /// The date the item was created
    date: DateTime<Utc>,
}

impl InnerItem {
    pub fn cid(&self) -> Cid {
        self.cid
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn date(&self) -> &DateTime<Utc> {
        &self.date
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl InnerItem {
    pub fn from_disk(path: &std::path::Path) -> KrondorResult<Self> {
        let cid = Cid::from_file(path)?;
        let date = DateTime::from(Utc::now());
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        Ok(Self { cid, date, name })
    }
}

// TODO: This should be a trait or something

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Post(PostItem),
    Gallery(GalleryItem),
}

impl Item {
    pub fn cid(&self) -> Cid {
        match self {
            Item::Post(i) => i.inner.cid(),
            Item::Gallery(i) => i.inner.cid(),
        }
    }

    pub fn collection(&self) -> &str {
        match self {
            Item::Post { .. } => POST_COLLECTION,
            Item::Gallery { .. } => GALLERY_COLLECTION,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Item::Post(i) => i.inner.name(),
            Item::Gallery(i) => i.inner.name(),
        }
    }

    pub fn date(&self) -> &DateTime<Utc> {
        match self {
            Item::Post(i) => &i.inner.date(),
            Item::Gallery(i) => &i.inner.date(),
        }
    }
}

impl Serialize for Item {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut value = Value::default();
        value["cid"] = serde_json::to_value(self.cid()).unwrap();
        value["name"] = serde_json::to_value(self.name()).unwrap();
        value["date"] = serde_json::to_value(self.date()).unwrap();
        match self {
            Item::Post(i) => {
                value["title"] = serde_json::to_value(&i.title).unwrap();
                value["collection"] = serde_json::to_value(POST_COLLECTION).unwrap();
                value.serialize(serializer)
            }
            Item::Gallery(i) => {
                value["collection"] = serde_json::to_value(GALLERY_COLLECTION).unwrap();
                value["description"] = serde_json::to_value(&i.description).unwrap();
                value.serialize(serializer)
            }
        }
    }
}

impl<'de> Deserialize<'de> for Item {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        let collection = value["collection"].as_str().ok_or_else(|| {
            serde::de::Error::custom("collection field is missing or not a string")
        })?;
        let value = value.clone();
        let inner = serde_json::from_value::<InnerItem>(value.clone()).map_err(serde::de::Error::custom)?;

        match collection {
            POST_COLLECTION => Ok(Item::Post(PostItem {
                inner,
                title: value["title"].as_str().unwrap_or_default().to_string()
            })),
            GALLERY_COLLECTION => Ok(Item::Gallery(GalleryItem {
                inner,
                description: value["description"].as_str().unwrap_or_default().to_string(),
            })),
            _ => Err(serde::de::Error::custom("Invalid collection")),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct PostItem {
    #[serde(flatten)]
    pub inner: InnerItem,
    pub title: String,
}

#[cfg(not(target_arch = "wasm32"))]
impl PostItem {
    pub fn new(path: &Path, title: &str) -> KrondorResult<Self> {
        let inner = InnerItem::from_disk(path)?;
        Ok(Self {
            inner,
            title: title.to_string(),
        })
    }
}

impl PostItem {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn collection(&self) -> &str {
        POST_COLLECTION
    }

    pub fn name(&self) -> &str {
        &self.inner.name
    }

    pub fn cid(&self) -> Cid {
        self.inner.cid
    }

    pub fn date(&self) -> &DateTime<Utc> {
        &self.inner.date
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct GalleryItem {
    #[serde(flatten)]
    pub inner: InnerItem,
    pub description: String,
}

impl GalleryItem {
    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn collection(&self) -> &str {
        GALLERY_COLLECTION
    }

    pub fn name(&self) -> &str {
        &self.inner.name
    }

    pub fn cid(&self) -> Cid {
        self.inner.cid
    }

    pub fn date(&self) -> &DateTime<Utc> {
        &self.inner.date
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl GalleryItem {
    pub fn new(path: &Path, description: &str) -> KrondorResult<Self> {
        let inner = InnerItem::from_disk(path)?;
        Ok(Self {
            inner,
            description: description.to_string(),
        })
    }
}