use std::{error::Error, path::PathBuf};
use clap::Args;
use log::{ info};
use schemars::schema_for;

use crate::xp::Xp;

#[derive(Args, Debug)]
pub struct GenerateSchemaArgs {
    /// path for generated json schema
    #[arg(short, long, default_value = "xp.schema.json")]
    schema_path: PathBuf,
}

pub fn run(args: &GenerateSchemaArgs) -> Result<(), Box<dyn Error>> {
    let schema = schema_for!(Xp);
    let schema_json = serde_json::to_string_pretty(&schema)?;
    info!("xp schema: {}", schema_json);
    std::fs::write(args.schema_path.clone(), schema_json)?;
    info!("wrote schema file to {}", args.schema_path.display());
    Ok(())
}