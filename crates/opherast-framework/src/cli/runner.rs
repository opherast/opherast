use crate::cli::builtin_commands;
use color_eyre::Result;

pub fn run_cli() -> Result<()> {
    color_eyre::install()?;

    let args: Vec<String> = std::env::args().skip(1).collect();
    let cmd = args.get(0).map(|s| s.as_str()).unwrap_or("help");

    for command in builtin_commands() {
        if command.name() == cmd {
            return command.run(&args[1..]);
        }
    }

    eprintln!("❌ Unknown command '{}'", cmd);
    std::process::exit(1);
}
