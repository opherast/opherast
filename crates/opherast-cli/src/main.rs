mod generated {
    include!(concat!(env!("OUT_DIR"), "/builtin_commands.rs"));
}
pub use generated::builtin_commands;
pub mod commands;

fn main() {
    if let Err(err) = opherast_core::kernel::run_cli(builtin_commands()) {
        eprintln!("❌ {}", err);
        std::process::exit(1);
    }
}
