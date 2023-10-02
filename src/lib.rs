mod error;
mod types;
mod common;

#[cfg(not(target_arch = "wasm32"))]
mod cli;

#[cfg(target_arch = "wasm32")]
mod web;

pub mod prelude {
    pub use crate::common::*;
    pub use crate::error::{KrondorError, KrondorResult};
    pub use crate::types::{Cid, Post, GalleryItem, Manifest, History, Config};

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
