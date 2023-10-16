use clap::{arg, command, Args, Subcommand};

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
    New(NewSubcommand),
}

#[derive(Debug, Args, Clone)]
pub struct NewSubcommand {
    /// Name of the new post
    #[arg(short, long)]
    pub name: String,
    /// Title of the new post
    #[arg(short, long)]
    pub title: String,
    /// Description of the new gallery
    #[arg(short, long)]
    pub description: String,
}