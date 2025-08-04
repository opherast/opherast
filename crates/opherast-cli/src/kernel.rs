mod generated {
    include!(concat!(env!("OUT_DIR"), "/builtin_commands.rs"));
}
use color_eyre::Result;
use once_cell::sync::OnceCell;

use crate::command::Command;

pub use generated::builtin_commands;

static COMMAND_NAMES: OnceCell<Vec<&'static str>> = OnceCell::new();

pub fn command_names() -> &'static [&'static str] {
    COMMAND_NAMES
        .get()
        .map(|v| v.as_slice())
        .unwrap_or(&[])
}

pub fn run_cli(commands: Vec<Box<dyn Command>>) -> Result<()> {
    color_eyre::install()?;
    let names: Vec<&'static str> = commands.iter().map(|c| c.name()).collect();
    let _ = COMMAND_NAMES.set(names);

    let args: Vec<String> = std::env::args().skip(1).collect();
    let cmd = args.get(0).map(|s| s.as_str()).unwrap_or("help");

    for command in commands {
        if command.name() == cmd {
            return command.run(&args[1..]);
        }
    }
    eprintln!("❌ Unknown command '{}'", cmd);
    std::process::exit(1);
}
