fn main() {
    if let Err(err) = opherast_framework::cli::runner::run_cli() {
        eprintln!("❌ {}", err);
        std::process::exit(1);
    }
}
