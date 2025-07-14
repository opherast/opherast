pub mod command;
pub mod commands;
pub mod runner;


pub use command::Command;
use commands::check::CheckCommand;
use commands::make_feature::MakeFeatureCommand;
use commands::list::ListCommand;

pub fn builtin_commands() -> Vec<Box<dyn Command>> {
    vec![
        Box::new(CheckCommand),
        Box::new(MakeFeatureCommand),
        Box::new(ListCommand)
    ]
}
