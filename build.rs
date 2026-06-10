use std::fs;

fn main() {
    println!("cargo:rerun-if-changed=migrations");

    if let Ok(entries) = fs::read_dir("migrations") {
        for entry in entries.flatten() {
            if let Some(path) = entry.path().to_str() {
                println!("cargo:rerun-if-changed={}", path);
            }
        }
    }
}
