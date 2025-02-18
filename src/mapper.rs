use crate::randomizer::Randomizer;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Mapper {
    pub files: Vec<String>,
    pub mapping: HashMap<String, String>,
    pub path: PathBuf,
    pub randomizer: Randomizer,
}

impl Mapper {
    pub fn new(path: PathBuf) -> Self {
        Self {
            randomizer: Randomizer::new(),
            files: Vec::new(),
            mapping: HashMap::new(),
            path,
        }
    }

    pub fn map(&mut self, files: Vec<String>) -> HashMap<String, String> {
        let random_names: Vec<String> = self.randomizer.randomize(&files);
        for (i, file) in files.iter().enumerate() {
            let file_dir_path = file.clone();
            // we're splitting the path by / and taking all but the last element
            let file_dir_path = file_dir_path
                .split("/")
                .collect::<Vec<&str>>()
                .into_iter()
                .take(file_dir_path.split("/").count() - 1)
                .collect::<Vec<&str>>()
                .join("/");
            if !self.mapping.contains_key(&file.clone()) {
                self.mapping.insert(file.clone(), random_names[i].clone());
            }
        }
        self.mapping.clone()
    }

    pub fn restore(&mut self, mapping: HashMap<String, String>) -> HashMap<String, String> {
        let mut reversed_map = HashMap::new();
        for (original_path, random_name) in mapping {
            let path = std::path::Path::new(&original_path);
            let parent = path
                .parent()
                .unwrap_or(std::path::Path::new(""))
                .to_str()
                .unwrap_or("");

            let random_path = format!("{}/{}.py", parent, random_name);
            reversed_map.insert(random_path, original_path);
        }
        reversed_map
    }

    pub fn get_files(&self, path: &str) -> std::io::Result<Vec<String>> {
        let mut files = Vec::new();
        self.get_files_recursive(path, &mut files)?;
        println!("{:?}", files.clone());
        Ok(files)
    }

    fn get_files_recursive(&self, path: &str, files: &mut Vec<String>) -> std::io::Result<()> {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().map_or(false, |ext| ext == "py") {
                if let Some(path_str) = path.to_str() {
                    files.push(String::from(path_str));
                }
            } else if path.is_dir() {
                if let Some(path_str) = path.to_str() {
                    self.get_files_recursive(path_str, files)?;
                }
            }
        }
        Ok(())
    }

    pub fn save_mapping(&self, path: &str) -> std::io::Result<()> {
        let json = serde_json::to_string(&self.mapping)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn load_mapping(&mut self, path: &str) -> std::io::Result<HashMap<String, String>> {
        let mapping_path = format!("{}/mappings.json", path);
        let json = fs::read_to_string(mapping_path)?;
        self.mapping = serde_json::from_str(&json)?;
        Ok(self.mapping.clone())
    }
}
