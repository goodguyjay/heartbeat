use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

#[derive(Deserialize, Debug)]
pub struct DialogTree {
    pub intro: HashMap<String, serde_json::Value>,
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

    pub fn get(&self, section: &str, key: &str) -> Option<&serde_json::Value> {
        if section == "intro" {
            self.intro.get(key)
        } else {
            None
        }
    }

    pub fn display_dialog(&self, section: &str, key: &str) {
        if let Some(value) = self.get(section, key) {
            if let Some(text) = value.as_str() {
                for char in text.chars() {
                    print!("{}", char);
                    std::io::stdout().flush().unwrap();
                    sleep(Duration::from_millis(5));
                }
                println!(); // Add a newline after the text
            }
        } else {
            eprintln!("Dialog not found for section '{}' and key '{}'", section, key);
        }
    }
}
