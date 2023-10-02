use clap::{command, Parser, Subcommand};
use std::path::Path;
use crate::prelude::*;

pub struct App {
    args: Args,
    config: Option<Config>,
}

impl App {
    pub fn new() -> Self {
        let config = match Config::load(Path::new(CONFIG_PATH)) {
            Ok(config) => Some(config),
            Err(_) => None,
        };
        let args = Args::parse();
        Self {
            args,
            config,
        }
    }

    pub fn run(&self) {
        match self.args.command {
            Command::Init => {
                let config = Config::new();
                init_space(Path::new(&config.path)).map_err(|e| {
                    println!("Path is already initialized: {}", e);
                }).ok();
                config.save(Path::new("krondor.json")).unwrap();
            }
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Command passed
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand, Clone)]
enum Command {
    Init,
}

// Initialize the path with the necessary directories and files
// - data
//   - manifest.json
//   - history.json
// - posts
// - gallery
// If any files or directories already exist, throw an error
fn init_space(path: &Path) -> KrondorResult<()> {
    let data_path = path.join(DATA_PATH);
    let posts_path = data_path.join(POSTS_PATH);
    let gallery_path = data_path.join(GALLERY_PATH);
    let manifest_path = data_path.join(MANIFEST_FILE);
    let history_path = data_path.join(HISTORY_FILE);

    if data_path.exists() {
        return Err(anyhow::anyhow!("Data directory already exists"));
    }
    if posts_path.exists() {
        return Err(anyhow::anyhow!("Posts directory already exists"));
    }
    if gallery_path.exists() {
        return Err(anyhow::anyhow!("Gallery directory already exists"));
    }
    if manifest_path.exists() {
        return Err(anyhow::anyhow!("Manifest file already exists"));
    }
    if history_path.exists() {
        return Err(anyhow::anyhow!("History file already exists"));
    }

    std::fs::create_dir_all(&data_path)?;
    std::fs::create_dir_all(&posts_path)?;
    std::fs::create_dir_all(&gallery_path)?;

    let manifest = Manifest::new();
    let history = History::new();

    manifest.save(&manifest_path)?;
    history.save(&history_path)?;

    Ok(())
}