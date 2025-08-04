use color_eyre::Result;

pub trait Command {
    fn name(&self) -> &'static str;
    fn run(&self, args: &[String]) -> Result<()>;
}
