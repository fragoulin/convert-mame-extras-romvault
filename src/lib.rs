pub mod dat;
pub mod files;
pub mod zip;

use crate::{dat::generate_output, files::cleanup_temp_dir, zip::check_input_file};
use anyhow::anyhow;
use files::extract_version;
use std::{env::temp_dir, path::PathBuf};

type Result<T> = anyhow::Result<T>;

pub struct Config {
    input_file_path: String,
    output_file_path: String,
    version: f32,
    pub temp_dir_path: PathBuf,
}

impl Config {
    /// # Errors
    ///
    /// Will return `Err` if input file or output file arguments are missing.
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config> {
        args.next();

        let Some(input_file_path) = args.next() else {
            return Err(anyhow!("missing input file"));
        };
        let Some(output_file_path) = args.next() else {
            return Err(anyhow!("missing output file"));
        };

        let version = extract_version(&input_file_path).unwrap_or(0.01);

        let temp_dir_path = temp_dir();

        Ok(Config {
            input_file_path,
            output_file_path,
            version,
            temp_dir_path,
        })
    }
}

/// # Errors
///
/// Will return `Err` if input file in config does not exist,
/// cannot be read or if it is an invalid zip file.
pub fn run(config: &Config) -> Result<()> {
    let mut archive = check_input_file(&config.input_file_path)?;

    println!(
        "Unpacking {} in {}",
        &config.input_file_path,
        &config.temp_dir_path.display()
    );

    if archive.extract(&config.temp_dir_path).is_err() {
        return Err(anyhow!(
            "failed to extract zip file {} into directory {}",
            String::from(&config.input_file_path),
            &config.temp_dir_path.display()
        ));
    }

    println!(
        "Generating {} for version {}",
        config.output_file_path, config.version
    );

    generate_output(&config.output_file_path, config.version)?;

    println!("Cleanup files in {}", config.temp_dir_path.display());

    cleanup_temp_dir(&config.temp_dir_path)?;

    Ok(())
}
