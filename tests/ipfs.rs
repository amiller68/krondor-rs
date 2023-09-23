use std::path::Path;
use tokio::fs;

use anyhow::Result;
use krondor::prelude::{GatewayClient, NodeClient};

const IPFS_URL: &str = "http://127.0.0.1:5001";
const GATEWAY_URL: &str = "http://127.0.0.1:8080";

fn new_node_client() -> NodeClient {
    NodeClient::new(IPFS_URL.to_string(), None, None)
}

async fn pin_read_file() -> Result<()> {
    let nc = new_node_client();
    let gc = GatewayClient(GATEWAY_URL.to_string());
    let file_data = fs::read_to_string("Cargo.toml").await?;
    let cid = nc.pin_file(Path::new("Cargo.toml")).await?;
    let data = gc.get(&cid).await?;
    assert_eq!(data, file_data);
    Ok(())
}

async fn pin_pull_file() -> Result<()> {
    let nc = new_node_client();
    let file_data = fs::read_to_string("Cargo.toml").await?;
    let cid = nc.pin_file(Path::new("Cargo.toml")).await?;
    let file_path = Path::new("Cargo_copy.toml");
    nc.pull_file(&cid, file_path).await?;
    let data = fs::read_to_string(file_path).await?;
    fs::remove_file(file_path).await?;
    assert_eq!(data, file_data);
    Ok(())
}

async fn pin_directory() -> Result<()> {
    let nc = new_node_client();
    let gc = GatewayClient(GATEWAY_URL.to_string());
    let dir_path = Path::new("src");
    let cid = nc.pin_directory(dir_path).await?;
    let _data = gc.get(&cid).await?;
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
mod rust {
    use super::*;

    #[tokio::test]
    async fn rust_pin_read_file() -> Result<()> {
        pin_read_file().await
    }

    #[tokio::test]
    async fn rust_pin_pull_file() -> Result<()> {
        pin_pull_file().await
    }

    #[tokio::test]
    async fn rust_pin_directory() -> Result<()> {
        pin_directory().await
    }
}
