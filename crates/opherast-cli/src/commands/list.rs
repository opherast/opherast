use crate::command::Command;
use crate::kernel;

pub struct ListCommand;

impl Command for ListCommand {
    fn name(&self) -> &'static str {
        "list"
    }
    fn run(&self, _args: &[String]) -> eyre::Result<()> {
        println!("📦 Available Commands:");
        for name in kernel::command_names() {
            println!(" - {}", name);
        }
        Ok(())
    }
}
