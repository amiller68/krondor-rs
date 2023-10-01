// mod config;
mod error;
mod ipfs;
mod types;

#[cfg(not(target_arch = "wasm32"))]
mod cli;

#[cfg(target_arch = "wasm32")]
mod web;

pub mod prelude {
    pub use crate::error::{KrondorError, KrondorResult};
    pub use crate::types::{Cid, Post};
    // pub use crate::eth::RootCid;

    // #[cfg(target_arch = "wasm32")]
    // pub use crate::config::KrondorConfig;
    #[cfg(not(target_arch = "wasm32"))]
    pub use crate::config::{KrondorConfig, OnDiskKrondorConfig};

    #[cfg(target_arch = "wasm32")]
    pub use crate::ipfs::GatewayClient;
    #[cfg(not(target_arch = "wasm32"))]
    pub use crate::ipfs::{NodeClient, GatewayClient};
}

pub mod app {
    // Interfaces
    #[cfg(not(target_arch = "wasm32"))]
    pub use crate::cli::App;
    #[cfg(target_arch = "wasm32")]
    pub use crate::web::App;
}
