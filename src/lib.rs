pub mod dat;
pub mod files;
pub mod zip;

use crate::{files::cleanup_temp_dir, zip::check_input_file};
use dat::generate_output;
use files::extract_version;
use std::{env, error::Error};

pub struct Config {
    input_file_path: String,
    output_file_path: String,
    version: f32,
}

impl Config {
    /// # Errors
    ///
    /// Will return `Err` if input file or output file arguments are missing.
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let Some(input_file_path) = args.next() else {
            return Err("Missing input file");
        };
        let Some(output_file_path) = args.next() else {
            return Err("Missing output file");
        };

        let version = extract_version(&input_file_path).unwrap_or(0.01);

        Ok(Config {
            input_file_path,
            output_file_path,
            version,
        })
    }
}

/// # Errors
///
/// Will return `Err` if input file in config does not exist,
/// cannot be read or if it is an invalid zip file.
pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let mut archive = check_input_file(&config.input_file_path)?;
    let temp_dir = env::temp_dir();

    println!(
        "Unpacking {} in {}",
        &config.input_file_path,
        temp_dir.display()
    );
    archive.extract(&temp_dir)?;

    println!(
        "Generating {} for version {}",
        config.output_file_path, config.version
    );
    generate_output(&config.output_file_path, config.version)?;

    println!("Cleanup files in {}", temp_dir.display());
    cleanup_temp_dir(&temp_dir)?;

    Ok(())
}
