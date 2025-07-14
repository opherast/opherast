mod generated {
    include!(concat!(env!("OUT_DIR"), "/builtin_commands.rs"));
}
pub use generated::builtin_commands;
fn main() {
    if let Err(err) = opherast_framework::cli::runner::run_cli(builtin_commands()) {
        eprintln!("❌ {}", err);
        std::process::exit(1);
    }
}
