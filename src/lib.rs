mod env;
pub mod error;
mod ipfs;
mod types;

#[cfg(not(target_arch = "wasm32"))]
mod cli;

#[cfg(target_arch = "wasm32")]
mod web;

pub mod app {
    // Interfaces
    #[cfg(not(target_arch = "wasm32"))]
    pub use crate::cli::App;
    #[cfg(target_arch = "wasm32")]
    pub use crate::web::App;
}
