use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("builtin_commands.rs");

    let commands_dir = Path::new("../opherast-framework/src/cli/commands");
    let mut output = String::new();

    output.push_str("use opherast_framework::cli::Command;\n");

    let mut init = String::from("pub fn builtin_commands() -> Vec<Box<dyn Command>> {\n    vec![\n");

    for entry in fs::read_dir(commands_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) != Some("rs") {
            continue;
        }

        let file_stem = path.file_stem().unwrap().to_string_lossy();
        if file_stem == "mod" {
            continue;
        }

        let module = file_stem.to_string();
        let type_name = format!("{}Command", to_pascal_case(&module));

        output.push_str(&format!("use opherast_framework::cli::commands::{}::{};\n", module, type_name));
        init.push_str(&format!("        Box::new({}),\n", type_name));
    }

    init.push_str("    ]\n}\n");

    output.push_str("\n");
    output.push_str(&init);

    let mut file = File::create(dest_path)?;
    file.write_all(output.as_bytes())?;

    println!("cargo:rerun-if-changed=src/commands");

    Ok(())
}

fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut c = word.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str()
            }
        })
        .collect()
}
