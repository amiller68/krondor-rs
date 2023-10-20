// TODO: do something with this
use std::convert::{From, TryFrom};

use cid::Cid;
use leptos::{IntoView, View};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::error::{KrondorError, KrondorResult};

const SHA3_256_CODEC: u64 = 0x16;
const RAW_CODEC: u64 = 0x55;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SerializedCid(Cid);

impl std::fmt::Display for SerializedCid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.to_string().fmt(f)
    }
}

impl SerializedCid {
    pub fn from_str(s: &str) -> KrondorResult<Self> {
        let cid = Cid::try_from(s).map_err(KrondorError::default)?;
        Ok(Self(cid))
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_file(path: &std::path::Path) -> KrondorResult<Self> {
        use multihash::Multihash;
        use sha3::{Digest, Sha3_256};
        // Read the file as bytes
        // Open the file
        let mut file = std::fs::File::open(path).map_err(KrondorError::default)?;
        // Read the file
        let mut bytes = Vec::new();
        std::io::Read::read_to_end(&mut file, &mut bytes).map_err(KrondorError::default)?;
        // Hash the bytes
        let hash = Sha3_256::digest(&bytes);
        // Create the cid
        let mh = Multihash::wrap(SHA3_256_CODEC, &hash).map_err(KrondorError::default)?;
        let cid = Cid::new_v1(RAW_CODEC, mh);
        Ok(Self(cid))
    }
}

impl From<SerializedCid> for Cid {
    fn from(cid: SerializedCid) -> Self {
        cid.0
    }
}

impl IntoView for SerializedCid {
    fn into_view(self) -> View {
        use leptos::leptos_dom::Text;
        let text = self.to_string();
        let text = Text::new(text.into());
        text.into_view()
    }
}

impl Serialize for SerializedCid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'de> Deserialize<'de> for SerializedCid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(SerializedCid(Cid::try_from(s).unwrap()))
    }
}
