use clap::{Parser, Subcommand};
use crate::cli::{generate_schema::GenerateSchemaArgs, validate::ValidateArgs};

pub mod generate_schema;
pub mod validate;

#[derive(Parser)]
#[command(version, about, long_about = None)] 
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    GenerateSchema(GenerateSchemaArgs),
    Validate(ValidateArgs),
}
