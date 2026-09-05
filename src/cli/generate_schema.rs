use std::{error::Error, path::PathBuf};
use clap::Args;
use log::{ info};
use schemars::schema_for;

use crate::xp::Xp;

#[derive(Args, Debug)]
pub struct GenerateSchemaArgs {
    /// path to generate schema at
    #[arg(short, long, default_value = "xp.schema.json")]
    schema_path: PathBuf,
}

pub async fn run(args: &GenerateSchemaArgs) -> Result<(), Box<dyn Error>> {
    let schema = schema_for!(Xp);
    // todo: handle to_str error
    let schema_path = args.schema_path.to_str().unwrap();
    let schema_json = serde_json::to_string_pretty(&schema)?;
    info!("Xp schema: {}", schema_json);
    std::fs::write(schema_path, schema_json)?;
    info!("wrote schema file to {}", schema_path);
    Ok(())
}