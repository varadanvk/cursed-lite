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
    /// Randomize filenames in a directory
    Randomize {
        /// Directory path to randomize
        path: PathBuf,
    },
    /// Restore files using a mapping file
    Restore {
        /// Path to the mapping file
        path: PathBuf,
    },
}

fn new() -> Cli {
    Cli {
        command: Commands::Randomize {
            path: PathBuf::new(),
        },
    }
}
