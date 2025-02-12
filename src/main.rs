use clap::Parser;
use rand::Rng;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "pyfinder")]
#[command(about = "Lists all Python files and generates random names for them")]
struct Cli {
    #[clap(short, long)]
    path: String,
}

fn should_skip_directory(dir_name: &str) -> bool {
    let skip_dirs = ["venv", "env", ".env", ".venv", "__pycache__"];
    skip_dirs.contains(&dir_name)
}

fn find_python_files(dir: &Path, py_files: &mut Vec<String>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                // Skip virtual environment directories
                if path.is_dir() {
                    if let Some(dir_name) = path.file_name() {
                        if let Some(dir_str) = dir_name.to_str() {
                            if should_skip_directory(dir_str) {
                                continue;
                            }
                        }
                    }
                    find_python_files(&path, py_files);
                } else if let Some(extension) = path.extension() {
                    if extension == "py" {
                        if let Some(file_path) = path.to_str() {
                            py_files.push(file_path.to_string());
                        }
                    }
                }
            }
        }
    }
}

fn generate_random_name() -> String {
    let mut rng = rand::thread_rng();
    let letters = "abcdefghijklmnopqrstuvwxyz";
    let letters: Vec<char> = letters.chars().collect();
    let mut random_name = String::new();

    for _ in 0..10 {
        let idx = rng.gen_range(0..letters.len());
        random_name.push(letters[idx]);
    }

    random_name + ".py"
}

fn main() {
    let args = Cli::parse();
    let path = Path::new(&args.path);

    println!("Searching in directory: {}", path.display());

    let mut py_files = Vec::new();
    find_python_files(path, &mut py_files);

    if py_files.is_empty() {
        println!("No Python files found in the directory.");
    } else {
        println!("\nFound Python files with generated names:");
        for file in &py_files {
            let new_name = generate_random_name();
            println!("  {} -> {}", file, new_name);
        }
        println!("\nTotal Python files found: {}", py_files.len());
    }
}
