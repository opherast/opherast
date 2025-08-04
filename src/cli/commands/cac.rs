use opherast_core::command::Command;
use opherast_core::kernel;

pub struct CacCommand;

impl Command for CacCommand {
    fn name(&self) -> &'static str {
        "cac:list"
    }
    fn run(&self, _args: &[String]) -> eyre::Result<()> {
        println!("📦 Available Commands CAC:");
        for name in kernel::command_names() {
            println!(" - {}", name);
        }
        Ok(())
    }
}
