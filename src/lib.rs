mod config;
mod error;
mod eth;
mod ipfs;
mod types;

#[cfg(target_arch = "wasm32")]
mod web;

pub mod prelude {
    pub use crate::config::config;
    pub use crate::error::{KrondorError, KrondorResult};
    pub use crate::types::{Cid, Post};
    pub use crate::eth::RootCid;
    #[cfg(target_arch = "wasm32")]
    pub use crate::ipfs::GatewayClient;
    #[cfg(not(target_arch = "wasm32"))]
    pub use crate::ipfs::{NodeClient, GatewayClient};
    #[cfg(target_arch = "wasm32")]
    pub use crate::web::App;
}
