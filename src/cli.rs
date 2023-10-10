mod app;
mod fs;

pub use app::*;

use crate::error::KrondorResult;
use crate::types::{Manifest, Item, PostItem, GalleryItem, POST_COLLECTION_DIR, GALLERY_COLLECTION_DIR};

impl app::App {
    pub fn new() -> Self {
        let args = app::AppArgs::parse();
        Self { args }
    }

    pub fn run(&self) -> KrondorResult<()> {
        match &self.args.command {
            Command::Init => {
                fs::init_space().map_err(|e| {
                    println!("Path is already initialized: {}", e);
                }).ok();
                Ok(())
            }
            Command::New(new) => match new {
                NewSubcommand::Post(post) => {
                    let mut manifest = Manifest::load(&fs::manifest_path()).unwrap();
                    let path = fs::post_path(&post.name);
                    let item = PostItem::new(&path,  &post.title)?;
                    println!("Created post: {:?}", item);
                    manifest.add_item(Item::Post(item));
                    println!("Manifest: {:?}", manifest);
                    manifest.save(&fs::manifest_path()).unwrap();
                    Ok(())
                }
                NewSubcommand::Gallery(gallery) => {
                    let mut manifest = Manifest::load(&fs::manifest_path()).unwrap();
                    let path = fs::gallery_path(&gallery.name);
                    let item =  GalleryItem::new(&path, &gallery.description)?;
                    println!("Created gallery: {:?}", item);
                    manifest.add_item(Item::Gallery(item));
                    manifest.save(&fs::manifest_path()).unwrap();
                    Ok(())
                }
            }
        }
    }
}

