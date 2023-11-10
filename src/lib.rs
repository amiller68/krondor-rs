mod env;
mod error;
mod types;
pub mod utils;

#[cfg(not(target_arch = "wasm32"))]
pub mod cli;

#[cfg(target_arch = "wasm32")]
pub mod web;

#[cfg(not(target_arch = "wasm32"))]
pub use cli::App;

#[cfg(target_arch = "wasm32")]
pub use web::App;
