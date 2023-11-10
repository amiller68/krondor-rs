pub use super::args::{Args, Command, Parser};

use crate::error::KrondorResult;
use crate::types::{Item, Manifest};

pub struct App;

impl App {
    pub fn run() {
        capture_error(Self::run_result()).ok();
    }

    fn run_result() -> KrondorResult<()> {
        let args = Args::parse();

        match args.command {
            Command::Init => {
                println!("Initializing new space...");
                Manifest::new_space()
            }
            Command::New => {
                println!("Creating new post...");
                let mut manifest = Manifest::load()?;
                // Prompt for name, title, description
                let name = prompt_string("Name");
                let title = prompt_string("Title");
                let description = prompt_string("Description");
                let item = Item::from_disk(&name, &title, &description)?;

                println!("Cid: {}", item.cid());
                println!("Detected Render: {}", item.render());

                if prompt_bool("Continue? (y/n)") {
                    manifest.add_item(item);
                    manifest.save()?;
                }
                Ok(())
            }
        }
    }
}

fn capture_error<T>(result: KrondorResult<T>) -> KrondorResult<T> {
    match result {
        Ok(v) => Ok(v),
        Err(e) => {
            println!("Error Occured: {}", e);
            Err(e)
        }
    }
}

fn prompt_string(prompt: &str) -> String {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    print!("{}: ", prompt);
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    s.trim().to_string()
}

fn prompt_bool(prompt: &str) -> bool {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    print!("{}: ", prompt);
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    s.trim().to_lowercase() == "y"
}
