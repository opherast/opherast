#[test]
fn cli_list_outputs_commands() {
    let exe = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target/debug/opherast-cli")
        .to_string_lossy()
        .into_owned();
    let output = std::process::Command::new(exe)
        .arg("list")
        .output()
        .expect("failed to run cli");
    assert!(output.status.success(), "process failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Available Commands"));
    assert!(stdout.contains("list"));
    assert!(stdout.contains("check:env"));
    assert!(stdout.contains("make:feature"));
}
