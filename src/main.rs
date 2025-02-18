use clap::Parser;
use std::fs;
use std::{path::Path, process};

mod cli;
mod mapper;
mod randomizer;

fn main() {
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::Randomize { path } => {
            let mut mapper = mapper::Mapper::new(path.clone());
            let path_str = path.to_str().unwrap();
            let files = mapper.get_files(path_str).unwrap();
            let mapping = mapper.map(files);
            println!("{:?}", mapping);

            for (original, new_name) in &mapping {
                let original_path = Path::new(original);
                let parent = original_path.parent().unwrap_or(Path::new(""));
                let new_path = parent.join(format!("{}.py", new_name));

                if let Err(e) = fs::rename(original, &new_path) {
                    eprintln!(
                        "Error renaming {} to {}: {}",
                        original,
                        new_path.display(),
                        e
                    );
                }
            }

            let mapping_path = format!("{}/mappings.json", path_str);
            if let Err(e) = mapper.save_mapping(&mapping_path) {
                eprintln!("Error saving mapping: {}", e);
            }

            println!("Randomization complete!");
        }
        cli::Commands::Restore { path } => {
            let mut mapper = mapper::Mapper::new(path.clone());
            let path_str = path.to_str().unwrap();
            let old_mappings = mapper.load_mapping(path_str).unwrap();
            let mapping = mapper.restore(old_mappings);
            println!("{:?}", mapping);

            for (original, new_name) in &mapping {
                let original_path = Path::new(original);
                let parent = original_path.parent().unwrap_or(Path::new(""));
                let new_path = parent.join(format!("{}.py", new_name));
                let new_path = Path::new(new_name);

                if let Err(e) = fs::rename(original, &new_path) {
                    eprintln!(
                        "Error renaming {} to {}: {}",
                        original,
                        new_path.display(),
                        e
                    );
                }
            }

            let mapping_path = format!("{}/mappings.json", path_str);
            if let Err(e) = mapper.save_mapping(&mapping_path) {
                eprintln!("Error saving mapping: {}", e);
            }

            println!("Randomization complete!");
        }
    }
}
