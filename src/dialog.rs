use rand::seq::SliceRandom;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;
use tera::{Context, Tera};

#[derive(Deserialize, Debug)]
pub struct DialogTree {
    pub sections: HashMap<String, serde_json::Value>,
    #[serde(skip)]
    tera: Tera, // Tera is not deserializable, so we need to skip it
}

impl DialogTree {
    fn get_localization_path(file_path: &str) -> PathBuf {
        // If the development path exists, use it instead
        let dev_path = PathBuf::from(file_path);

        if dev_path.exists() {
            dev_path
        } else {
            let out_dir = std::env::var("OUT_DIR").unwrap();
            PathBuf::from(out_dir).join(file_path)
        }
    }

    // This function loads the JSON file and parses it into a HashMap
    pub fn load(file_name: &str) -> Self {
        let path = Self::get_localization_path(file_name);
        let content = fs::read_to_string(&path).unwrap_or_else(|_| {
            panic!("Failed to read dialog file at {:?}", path);
        });

        let dialog_data: HashMap<String, serde_json::Value> = serde_json::from_str(&content)
            .unwrap_or_else(|e| {
                panic!("Failed to parse JSON in dialog file at {:?}: {}", path, e);
            });

        Self {
            sections: dialog_data,
            tera: Tera::default(),
        }
    }

    // This function gets a dialog from the json file based on the section and key
    pub fn get(&self, section: &str, key: &str) -> Option<String> {
        if let Some(value) = self.sections.get(section) {
            if let Some(sub_value) = value.get(key) {
                match sub_value {
                    serde_json::Value::String(s) => Some(s.clone()),
                    serde_json::Value::Object(map) => {
                        // If the value is an object, we'll concatenate all the sub-keys
                        let mut parts: Vec<_> = map.iter().collect();
                        parts.sort_by_key(|(k, _)| *k);

                        Some(
                            parts
                                .into_iter()
                                .map(|(_, v)| v.as_str().unwrap_or_default())
                                .collect::<Vec<_>>()
                                .join(" "),
                        )
                    }

                    serde_json::Value::Array(arr) => {
                        // If the value is an array, we'll choose one entry randomly
                        arr.choose(&mut rand::thread_rng())
                            .and_then(|v| v.as_str().map(|s| s.to_string()))
                    }

                    _ => None,
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    // This function will display a dialog from the JSON file
    // If parameters are provided, it will replace placeholders in the text
    //
    // The lib tera was included to make template strings easier
    pub fn display_dialog(
        &mut self,
        section: &str,
        key: &str,
        variables: Option<HashMap<String, String>>,
    ) {
        if let Some(template) = self.get(section, key) {
            let mut context = Context::new();

            if let Some(vars) = variables {
                for (key, value) in vars {
                    context.insert(&key, &value);
                }
            }

            match self.tera.render_str(&template, &context) {
                Ok(rendered) => {
                    for char in rendered.chars() {
                        print!("{}", char);
                        std::io::stdout().flush().unwrap();
                        sleep(Duration::from_millis(25));
                    }
                    // Add a newline after the dialog is displayed
                    println!();
                }
                Err(e) => eprintln!("Failed to render dialog: {}", e),
            }
        } else {
            eprintln!(
                "Dialog not found for section '{}' and key '{}'",
                section, key
            );
        }
    }
}
