use clap::{command, Parser, Subcommand};
pub struct App(Args);

impl App {
    pub fn new() -> Self {
        let args = Args::parse();
        Self(args)
    }

    pub fn run(&self) {
        match self.0.command {
            Command::Init => {
                println!("Init");
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