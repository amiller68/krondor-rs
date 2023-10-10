use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::env;
use crate::error::{KrondorError, KrondorResult};

use super::cid::Cid;
use super::item::Item;

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Manifest {
    pub version: String,
    pub items: Vec<Item>,
}

impl Manifest {
    pub fn item_by_collection_and_name(
        &self,
        collection: &str,
        name: &str,
    ) -> Option<Item> {
        self.items
            .iter()
            .find(|i| i.collection() == collection && i.name() == name)
            .cloned()
    }
    pub fn item_by_cid(&self, cid: &Cid) -> Option<Item> {
        self.items.iter().find(|i| i.cid() == *cid).cloned()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Manifest {
    pub fn new() -> Self {
        Self {
            version: env::APP_VERSION.to_string(),
            items: Vec::new(),
        }
    }
    
    pub fn add_item(&mut self, item: Item) {
        // Check if the item already exists
        if self.item_by_collection_and_name(item.collection(), item.name()).is_some() {
            return;
        }
        self.items.push(item);
    }
    pub fn load(path: &Path) -> KrondorResult<Self> {
        let json = std::fs::read_to_string(path).map_err(KrondorError::default)?;
        let manifest =
            serde_json::from_str::<Self>(&json).map_err(KrondorError::default)?;
        Ok(manifest)
    }
    pub fn save(&self, path: &Path) -> KrondorResult<()> {
        let json =
            serde_json::to_string_pretty(self).map_err(KrondorError::default)?;
        std::fs::write(path, json).map_err(KrondorError::default)?;
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
    pub fn load(path: &Path) -> KrondorResult<Self> {
        let json = std::fs::read_to_string(path).map_err(KrondorError::default)?;
        let history =
            serde_json::from_str::<Self>(&json).map_err(KrondorError::default)?;
        Ok(history)
    }
    pub fn save(&self, path: &Path) -> KrondorResult<()> {
        let json =
            serde_json::to_string_pretty(self).map_err(KrondorError::default)?;
        std::fs::write(path, json).map_err(KrondorError::default)?;
        Ok(())
    }
}
