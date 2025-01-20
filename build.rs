use std::fs;
use std::path::Path;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("localization");

    if !dest_path.exists() {
        fs::create_dir_all(&dest_path).unwrap();
    }

    for entry in fs::read_dir("localization").unwrap() {
        let entry = entry.unwrap();
        let dest_file = dest_path.join(entry.file_name());
        fs::copy(entry.path(), dest_file).unwrap();
    }
}
