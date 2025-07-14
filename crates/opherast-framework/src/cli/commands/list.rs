use crate::cli::command::Command;

pub struct ListCommandsCommand;

impl Command for ListCommandsCommand {
    fn name(&self) -> &'static str {
        "list"
    }
    fn run(&self, _args: &[String]) -> eyre::Result<()> {
        let cmds = crate::cli::builtin_commands();
        println!("📦 Comandi disponibili:");
        for cmd in cmds {
            println!(" - {}", cmd.name());
        }
        Ok(())
    }
}
