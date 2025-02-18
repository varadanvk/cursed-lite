use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Randomize { path: PathBuf },
    Restore { path: PathBuf },
}

fn new() -> Cli {
    Cli {
        command: Commands::Randomize {
            path: PathBuf::new(),
        },
    }
}
