use std::error::Error;
use clap::Parser;

use xp::cli::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut log_builder = env_logger::Builder::new();
    log_builder
        .filter_level(log::LevelFilter::Warn)
        .filter_module("xp", log::LevelFilter::Trace)
        .format_source_path(true)
        .init();

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
