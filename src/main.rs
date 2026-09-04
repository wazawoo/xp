use std::error::Error;
use clap::Parser;

use xp::cli::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {    
    let cli = Cli::parse();
    match &cli.command {
        Some(commands) => {
            match commands {
                Commands::Validate(args) => xp::cli::validate::run(args).await?,
            }
        },
        None => {
            panic!("no subcommand");
        }
    }
    Ok(())
}
