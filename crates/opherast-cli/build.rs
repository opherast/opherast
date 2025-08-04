use std::{
    env,
    error::Error,
    fs::{self, File},
    io::Write,
    path::Path
};

fn scan_commands(dir: &Path, prefix: &str) -> Result<(String, String), Box<dyn Error>> {
    let mut uses = String::new();
    let mut entries = String::new();
    let mut mod_rs = String::new();
    if dir.exists() {
        for entry in fs::read_dir(dir)? {
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
            uses.push_str(&format!("use {}::{}::{};\n", prefix, module, type_name));
            entries.push_str(&format!("        Box::new({}),\n", type_name));
            mod_rs.push_str(&format!("pub mod {};\n", module));
        }
    }
    let builtin = if uses.is_empty() && entries.is_empty() {
        String::new()
    } else {
        format!("{}--SPLIT--\n{}", uses, entries)
    };
    Ok((builtin, mod_rs))
}

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("builtin_commands.rs");
    let (cli_builtin, cli_mod) = scan_commands(Path::new("./src/commands"), "crate::commands")?;
    let (app_builtin, app_mod) = scan_commands(
        Path::new("../../src/cli/commands"),
        "opherast::cli::commands",
    )?;

    let mut uses = String::new();
    let mut entries = String::new();
    for part in [cli_builtin, app_builtin] {
        if part.is_empty() {
            continue;
        }
        if let Some((u, e)) = part.split_once("--SPLIT--\n") {
            uses.push_str(u);
            entries.push_str(e);
        } else {
            uses.push_str(&part);
        }
    }
    let mut output = String::new();
    output.push_str("use opherast_core::command::Command;\n");
    output.push_str(&uses);
    output.push_str("\n");
    output.push_str("pub fn builtin_commands() -> Vec<Box<dyn Command>> {\n    vec![\n");
    output.push_str(&entries);
    output.push_str("    ]\n}\n");

    File::create(dest_path)?.write_all(output.as_bytes())?;

    fs::write(Path::new("./src/commands").join("mod.rs"), cli_mod)?;
    fs::write(Path::new("../../src/cli/commands").join("mod.rs"), app_mod)?;

    println!("cargo:rerun-if-changed=./src/commands");
    println!("cargo:rerun-if-changed=../../src/cli/commands");
    Ok(())
}

fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut c = word.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect()
}
