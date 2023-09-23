use anyhow::Result;
use crate::eth::RootCid;
#[cfg(not(target_arch = "wasm32"))]
use crate::ipfs::NodeClient;
#[cfg(target_arch = "wasm32")]
use crate::ipfs::GatewayClient;

#[cfg(not(target_arch = "wasm32"))]
use std::env;

const KRONDOR_JSON: &str = include_str!("../krondor.json");

pub fn config() -> Result<KrondorConfig> {
    KrondorConfig::new()
}

#[cfg(not(target_arch = "wasm32"))]
pub struct KrondorConfig {
    pub(crate) root_cid: RootCid,
    pub(crate) node: NodeClient,
}

#[cfg(target_arch = "wasm32")]
pub struct KrondorConfig {
    pub(crate) root_cid: RootCid,
    pub(crate) gateway: GatewayClient,
}

impl KrondorConfig {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new() -> Result<Self> {
        let config = serde_json::from_str::<serde_json::Value>(KRONDOR_JSON)?;
        let root_cid = RootCid::new(
            config["eth"]["rpc_url"].as_str().unwrap().to_string(),
            config["eth"]["contract_address"]
                .as_str()
                .unwrap()
                .to_string(),
            config["eth"]["chain_id"].as_u64().unwrap(),
            Some(env::var("PRIVATE_KEY").unwrap()),
        )?;
        let node = NodeClient::new(
            config["ipfs"]["endpoint"].as_str().unwrap().to_string(),
            Some(env::var("IPFS_KEY").unwrap()),
            Some(env::var("IPFS_SECRET").unwrap()),
        );
        Ok(Self { root_cid, node })
    }
    #[cfg(target_arch = "wasm32")]
    pub fn new() -> Result<Self> {
        let config = serde_json::from_str::<serde_json::Value>(KRONDOR_JSON)?;
        let root_cid = RootCid::new(
            config["eth"]["rpc_url"].as_str().unwrap().to_string(),
            config["eth"]["contract_address"]
                .as_str()
                .unwrap()
                .to_string(),
            config["eth"]["chain_id"].as_u64().unwrap(),
            None,
        )?;
        let gateway = GatewayClient(config["ipfs"]["gateway"].as_str().unwrap().to_string());
        Ok(Self { root_cid, gateway })
    }
}
