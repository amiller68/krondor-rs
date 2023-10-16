#[cfg(not(target_arch = "wasm32"))]
use {
    std::path::Path,
    crate::error::{KrondorError, KrondorResult},
    crate::env,
};

use serde::{Deserialize, Serialize};

use super::item::Item;

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Manifest {
    pub version: String,
    pub items: Vec<Item>,
}

impl Manifest {
    pub fn item(&self, name: &str) -> Option<Item> {
        self.items
            .iter()
            .find(|i| i.name() == name)
            .cloned()
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
        if self
            .item(item.name())
            .is_some()
        {
            return;
        }
        self.items.push(item);
    }
    pub fn load(path: &Path) -> KrondorResult<Self> {
        let json = std::fs::read_to_string(path).map_err(KrondorError::default)?;
        let manifest = serde_json::from_str::<Self>(&json).map_err(KrondorError::default)?;
        Ok(manifest)
    }
    pub fn save(&self, path: &Path) -> KrondorResult<()> {
        let json = serde_json::to_string_pretty(self).map_err(KrondorError::default)?;
        std::fs::write(path, json).map_err(KrondorError::default)?;
        Ok(())
    }
}