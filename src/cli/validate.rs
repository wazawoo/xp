use std::{error::Error, path::PathBuf};
use clap::Args;
use log::{error, info};

use crate::xp::Xp;

#[derive(Args, Debug)]
pub struct ValidateArgs {
    /// path to xp file (json)
    #[arg(short, long)]
    xp_file_path: PathBuf,
}

pub async fn run(args: &ValidateArgs) -> Result<(), Box<dyn Error>> {
    let file = std::fs::read_to_string(&args.xp_file_path)?;
    match eserde::json::from_str::<Xp>(&file) {
        Ok(xp) => {
            info!("validate Ok: {xp:#?}");
            Ok(())
        }
        Err(e) => {
            error!("validate Err: {e}");
            std::process::exit(1);
        }
    }
}