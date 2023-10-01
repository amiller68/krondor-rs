// use std::path::Path;

// use anyhow::Result;
// use serde::{Deserialize, Serialize};
// use serde_json::Value;

// use crate::eth::RootCid;
// #[cfg(not(target_arch = "wasm32"))]
// use crate::ipfs::NodeClient;
// #[cfg(target_arch = "wasm32")]
// use crate::ipfs::GatewayClient;

// #[cfg(not(target_arch = "wasm32"))]
// use std::env;
// #[cfg(target_arch = "wasm32")]
// const KRONDOR_JSON: &str = include_str!("../krondor.json");

// #[cfg(not(target_arch = "wasm32"))]
// #[derive(serde::Deserialize, Serialize)]
// pub struct OnDiskKrondorConfig {
//     eth_rpc_url: String,
//     eth_contract_address: String,
//     eth_chain_id: u64,
//     ipfs_endpoint: String,
//     ipfs_gateway: String,
// }

// #[cfg(not(target_arch = "wasm32"))]
// impl OnDiskKrondorConfig {
//     pub fn save(&self, path: &str) -> Result<()> {
//         let json = serde_json::to_string_pretty(self)?;
//         std::fs::write(path, json)?;
//         Ok(())
//     }

//     pub fn load(path: &str) -> Result<Self> {
//         let json = std::fs::read_to_string(path)?;
//         let config = serde_json::from_str::<Self>(&json)?;
//         Ok(config)
//     }
// }

// #[cfg(not(target_arch = "wasm32"))]
// pub struct KrondorConfig {
//     pub(crate) root_cid: RootCid,
//     pub(crate) node: NodeClient,
// }

// #[cfg(target_arch = "wasm32")]
// pub struct KrondorConfig {
//     // pub(crate) root_cid: RootCid,
//     pub(crate) gateway: GatewayClient,
// }

// impl KrondorConfig {
//     #[cfg(not(target_arch = "wasm32"))]
//     pub fn load(
//         path: &Path
//     ) -> Result<Self> {
//         let config = OnDiskKrondorConfig::load(path.to_str().unwrap())?;
//         let root_cid = RootCid::new(
//             config.eth_rpc_url,
//             config.eth_contract_address,
//             config.eth_chain_id,
//             Some(env::var("PRIVATE_KEY").unwrap()),
//         )?;
//         let node = NodeClient::new(
//             config.ipfs_endpoint,
//             Some(env::var("IPFS_KEY").unwrap()),
//             Some(env::var("IPFS_SECRET").unwrap()),
//         );
//         Ok(Self { root_cid, node })
//     }
// }
