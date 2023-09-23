use anyhow::Result;
use krondor::prelude::*;

const HARDHAT_KEY: &str = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
const HARDHAT_ADDRESS: &str = "0x5fbdb2315678afecb367f032d93f642f64180aa3";
const HARDHAT_URL: &str = "http://127.0.0.1:8545/";
const HARDHAY_CHAIN_ID: u64 = 31337;

fn new_root_cid() -> RootCid {
    RootCid::new(
        HARDHAT_URL.to_string(),
        HARDHAT_ADDRESS.to_string(),
        HARDHAY_CHAIN_ID,
        Some(HARDHAT_KEY.to_string()),
    )
    .unwrap()
}

async fn update_get_cid() -> Result<()> {
    let rc = new_root_cid();
    let _ = rc.update("test".to_string()).await.unwrap();
    let cid = rc.get().await.unwrap();
    assert_eq!(cid, "test");
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
mod rust {
    use super::*;

    #[tokio::test]
    async fn rust_update_get_cid() -> Result<()> {
        update_get_cid().await
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test(async)]
    async fn wasm_update_get_cid() -> Result<()> {
        update_get_cid().await
    }
}
