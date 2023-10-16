mod app;
mod fs;

pub use app::*;

use crate::error::KrondorResult;

impl app::App {
    pub fn new() -> Self {
        let args = app::AppArgs::parse();
        Self { args }
    }

    pub fn run(&self) -> KrondorResult<()> {
        match &self.args.command {
            Command::Init => {
                fs::init_space()
                    .map_err(|e| {
                        println!("Path is already initialized: {}", e);
                    })
                    .ok();
                Ok(())
            }
            Command::New(post) => {
                Ok(())
            }
        }
    }
}
