use std::{error::Error, path::PathBuf};
use clap::Args;

#[derive(Args, Debug)]
pub struct ValidateArgs {
    /// path to xp file (json)
    #[arg(short, long)]
    xp_file_path: PathBuf,
}

pub async fn run(args: &ValidateArgs) -> Result<(), Box<dyn Error>> {
    todo!("")
}