use std::convert::{From, TryFrom};

use cid::Cid as BaseCid;
use leptos::{IntoView, View};
use multihash::Multihash;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sha3::{Digest, Sha3_256};

use crate::error::{KrondorError, KrondorResult};

const SHA3_256_CODEC: u64 = 0x16;
const RAW_CODEC: u64 = 0x55;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cid(BaseCid);

impl Cid {
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    pub fn from_str(s: &str) -> KrondorResult<Self> {
        let cid = BaseCid::try_from(s).map_err(KrondorError::default)?;
        Ok(Self(cid))
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_file(path: &std::path::Path) -> KrondorResult<Self> {
        // Read the file as bytes
        // Open the file
        let mut file = std::fs::File::open(path).map_err(KrondorError::default)?;
        // Read the file
        let mut bytes = Vec::new();
        std::io::Read::read_to_end(&mut file, &mut bytes)
            .map_err(KrondorError::default)?;
        // Hash the bytes
        let hash = Sha3_256::digest(&bytes);
        // Create the cid
        let mh =
            Multihash::wrap(SHA3_256_CODEC, &hash).map_err(KrondorError::default)?;
        let cid = BaseCid::new_v1(RAW_CODEC, mh);
        Ok(Self(cid))
    }
}

impl From<Cid> for BaseCid {
    fn from(cid: Cid) -> Self {
        cid.0
    }
}

impl IntoView for Cid {
    fn into_view(self) -> View {
        use leptos::leptos_dom::Text;
        let text = self.to_string();
        let text = Text::new(text.into());
        text.into_view()
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
