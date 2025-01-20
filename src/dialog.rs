use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Debug)]
pub struct DialogTree {
    pub intro: HashMap<String, String>,
    pub anna: HashMap<String, String>,
    pub turn: HashMap<String, String>,
}

impl DialogTree {
    fn get_localization_path(file_path: &str) -> PathBuf {
        let dev_path = PathBuf::from(file_path);

        if dev_path.exists() {
            dev_path
        } else {
            let out_dir = std::env::var("OUT_DIR").unwrap();
            PathBuf::from(out_dir).join(file_path)
        }
    }

    pub fn load(file_name: &str) -> Self {
        let path = Self::get_localization_path(file_name);
        let content = fs::read_to_string(path).expect("Failed to read file");
        serde_json::from_str(&content).expect("Failed to parse JSON")
    }

    pub fn get(&self, section: &str, key: &str) -> &str {
        self.intro
            .get(key)
            .or_else(|| self.anna.get(key))
            .or_else(|| self.turn.get(key))
            .unwrap()
    }
}
