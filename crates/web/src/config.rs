use anyhow::Result;
use common::prelude::*;

// Statically import ../krondor.json
const KRONDOR_JSON: &str = include_str!("../../../krondor.json");

pub struct KrondorConfig {
    pub(crate) root_cid: RootCid,
    pub(crate) gateway: GatewayClient,
}

impl KrondorConfig {
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
