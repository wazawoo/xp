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

pub fn run(args: &ValidateArgs) -> Result<(), Box<dyn Error>> {
    let file = std::fs::read_to_string(&args.xp_file_path)?;
    match eserde::json::from_str::<Xp>(&file) {
        Ok(xp) => {
            info!("validate Ok: {xp:#?}");
            Ok(())
        }
        Err(e) => {
            // i'd rather return the error but i don't want it to print ugly
            error!("validate Err: {e}");
            std::process::exit(1);
            // Err(Box::new(e))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use assert_cmd::Command;

    #[test]
    fn examples_valid() -> Result<(), Box<dyn Error>> {
        let proj_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let examples_dir = proj_dir.join("examples");
        let example_paths = fs::read_dir(examples_dir)?;

        for path_res in example_paths {
            let example_path = path_res?.path();
            println!("Validating example .xp file: {}", example_path.display());
            let mut cmd = Command::cargo_bin("xp")?;
            cmd
                .arg("validate")
                .arg("--xp-file-path")
                .arg(example_path)
                .assert()
                .success();
        }
        Ok(())
    }
}
