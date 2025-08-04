mod kernel;
pub mod command;
pub mod commands;

fn main() {
    if let Err(err) = crate::kernel::run_cli(kernel::builtin_commands()) {
        eprintln!("❌ {}", err);
        std::process::exit(1);
    }
}
