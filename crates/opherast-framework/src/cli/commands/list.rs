use crate::cli::command::Command;

pub struct ListCommand;

impl Command for ListCommand {
    fn name(&self) -> &'static str {
        "list"
    }
    fn run(&self, _args: &[String]) -> eyre::Result<()> {
        println!("📦 Comandi disponibili:");
        for name in crate::cli::runner::command_names() {
            println!(" - {}", name);
        }
        Ok(())
    }
}
