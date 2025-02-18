use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Randomizer {
    pub random_names: Vec<String>,
}

impl Randomizer {
    pub fn new() -> Self {
        Self {
            random_names: Vec::new(),
        }
    }
    pub fn gen_name(&mut self, filename: &str) -> String {
        let letter_combos = String::from("abcdefghijklmnopqrstuvwxyz");
        let letter_list = letter_combos.chars().collect::<Vec<char>>();
        let mut rng = rand::thread_rng();

        let mut random_name = String::new();
        for _ in 0..10 {
            let random_index = rng.gen_range(0..letter_list.len());
            random_name.push(letter_list[random_index]);
        }
        random_name
    }

    pub fn randomize(&mut self, files: &Vec<String>) -> Vec<String> {
        let mut randomized_names: Vec<String> = Vec::new();
        for name in files {
            let random_name = self.gen_name(name);
            if !randomized_names.contains(&random_name) {
                randomized_names.push(random_name);
            }
        }

        randomized_names
    }
}
