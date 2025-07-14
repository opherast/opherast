use crate::cli::command::Command;
use std::env;

pub struct CheckCommand;

impl Command for CheckCommand {
    fn name(&self) -> &'static str {
        "check:env"
    }

    fn run(&self, _args: &[String]) -> eyre::Result<()> {
        dotenvy::dotenv().ok();
        println!("Running opherast check...\n");

        let mut ok = true;

        env::var("OPHERAST_WORKER_THREADS")
            .map(|val| match val.parse::<usize>() {
                Ok(n) if n > 0 => println!("OPHERAST_WORKER_THREADS = {n}"),
                Ok(_) => {
                    println!("OPHERAST_WORKER_THREADS must be > 0");
                    ok = false;
                }
                Err(_) => {
                    println!("OPHERAST_WORKER_THREADS is not a valid number");
                    ok = false;
                }
            })
            .unwrap_or_else(|_| {
                println!("OPHERAST_WORKER_THREADS not set (default = all cores)");
            });

        match env::var("DATABASE_URL") {
            Ok(url) if !url.is_empty() => println!("DATABASE_URL is set"),
            _ => {
                println!("DATABASE_URL is missing or empty");
                ok = false;
            }
        }

        if ok {
            println!("\nEnvironment looks good!");
            Ok(())
        } else {
            Err(eyre::eyre!("One or more checks failed"))
        }
    }
}
