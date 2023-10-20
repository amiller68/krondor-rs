mod app;
mod fs;

use app::{AppArgs, Parser, Command};
pub use app:: App;

use crate::error::KrondorResult;
use crate::types::{Manifest, Item, SerializedCid};

impl App {
    pub fn new() -> Self {
        let args = AppArgs::parse();
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
                let post_path = fs::post_path(&post.name);
                println!("Creating post at {}", post_path.display());
                let cid = match SerializedCid::from_file(&post_path) {
                    Ok(cid) => cid,
                    Err(e) => {
                        println!("Error: {}", e);
                        return Ok(());
                    }
                };
                let mut manifest = Manifest::load(&fs::manifest_path())?;
                let item = Item::new(
                    cid,
                    &post.name,
                    &post.title,
                    &post.description,
                );
                manifest.add_item(item);
                manifest.save(&fs::manifest_path())?;
                Ok(())
            }
        }
    }
}
