pub mod eth;
pub mod ipfs;

pub mod prelude {
    pub use crate::eth::RootCid;
    pub use crate::ipfs::GatewayClient;
    #[cfg(not(target_arch = "wasm32"))]
    pub use crate::ipfs::NodeClient;
}
