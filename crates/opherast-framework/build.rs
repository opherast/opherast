use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

fn main() {
    let commands_dir = Path::new("src/cli/commands");
    let mod_path = commands_dir.join("mod.rs");
    let mut mod_file = String::new();
    for entry in fs::read_dir(commands_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("rs") {
            continue;
        }
        let file_stem = path.file_stem().unwrap().to_string_lossy();
        if file_stem == "mod" {
            continue;
        }
        mod_file.push_str(&format!("pub mod {};\n", file_stem));
    }
    File::create(mod_path).unwrap().write_all(mod_file.as_bytes()).unwrap();
    println!("cargo:rerun-if-changed=src/cli/commands");
}
