use clap::{command, arg, Subcommand, Args};

pub use clap::Parser;

pub struct App {
    pub(crate) args: AppArgs,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct AppArgs {
    /// Command passed
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand, Clone)]
pub enum Command {
    Init,
    #[command(subcommand)]
    New(NewSubcommand),
}

#[derive(Debug, Subcommand, Clone)]
pub enum NewSubcommand {
    Post(PostSubcommand),
    Gallery(GallerySubcommand),
}

#[derive(Debug, Clone, Args)]
pub struct PostSubcommand {
    /// Name of the post
    #[arg(short, long)]
    pub name: String,
    /// Title of the post
    #[arg(short, long)]
    pub title: String,
}

#[derive(Debug, Parser, Clone)]
pub struct GallerySubcommand {
    /// Name of the gallery
    #[arg(short, long)]
    pub name: String,
    /// Description of the gallery
    #[arg(short, long)]
    pub description: String,
}