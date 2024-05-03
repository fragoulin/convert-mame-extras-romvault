//! # Convert MAME Extras Romvault
//!
//! This crate can be used to convert MAME Extras Zip file
//! to a compatible format to use with Romvault.

pub mod dat;
pub mod files;
pub mod zip;

use crate::{dat::generate_output, zip::check_input_file};
use anyhow::anyhow;
use files::extract_version;
use std::{path::PathBuf, time::Instant};
use temp_dir::TempDir;

/// Custom result with any context error.
type Result<T> = anyhow::Result<T>;

/// Main configuration to hold various parameters:
/// - Input file path
/// - Output file path
/// - Version computed from input file name
/// - Temporary directory path
pub struct Config {
    /// Zip file used for input.
    input_file_path: PathBuf,
    /// Generated dat will be written into this output file.
    output_file_path: PathBuf,
    /// Version extracted from input Zip file name. Will be used for dat generation.
    version: f32,
    /// Temporary directory to write dats extracted from Zip file into.
    temp_dir_path: TempDir,
}

impl Config {
    /// # Errors
    ///
    /// Will return `Err` if input file argument is missing.
    fn build(args: &[String]) -> Result<Self> {
        if args.is_empty() {
            return Err(anyhow!("missing input file"));
        }

        let input_file_path = PathBuf::from(&args[0]);
        let mut output_file_path: PathBuf;

        if args.len() == 1 {
            // No output file specified
            let input_file_name = input_file_path.file_name().unwrap();
            output_file_path = PathBuf::from(input_file_name);
            output_file_path.set_extension("dat");
        } else {
            output_file_path = PathBuf::from(&args[1]);
        }

        let file_name = input_file_path.display().to_string();
        let version = extract_version(file_name.as_str()).unwrap_or(0.01);

        let temp_dir_path = TempDir::new().unwrap();

        Ok(Self {
            input_file_path,
            output_file_path,
            version,
            temp_dir_path,
        })
    }
}

/// Parse arguments and tries to generate output dat file.
///
/// Returns 0 if no error occurred.
/// Returns 1 in case of error.
#[must_use]
pub fn real_main(args: &[String]) -> i32 {
    let now = Instant::now();

    let config_result = Config::build(args);
    let config = match config_result {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Problem parsing arguments: {err}",);
            print_usage();
            return 1;
        }
    };

    if let Err(err) = run(&config) {
        eprintln!("Error: {err}");
        return 1;
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {elapsed:.2?}");

    0
}

/// Print help message on stderr.
fn print_usage() {
    let indent = 7;
    eprintln!("Usage: convert-mame-extras-romvault <inputfile> <outputfile>");
    eprintln!("{:indent$}<inputfile> is mandatory and must be a valid Zip file (e.g. `MAME 0.264 EXTRAs.zip`)", "");
    eprintln!("{:indent$}<outputfile> is optional. If not specified, the name of the input file will be used (e.g. `MAME 0.264 EXTRAs.dat`)", "");
}

/// # Errors
///
/// Will return `Err` if input file in config does not exist,
/// cannot be read or if it is an invalid zip file.
fn run(config: &Config) -> Result<()> {
    let mut archive = check_input_file(&config.input_file_path)?;

    println!("Unpacking {}", &config.input_file_path.display(),);

    let result = archive.extract(config.temp_dir_path.path());
    if result.is_err() {
        return Err(anyhow!(
            "failed to extract zip file `{}` (`{}`)",
            String::from(&config.input_file_path.display().to_string()),
            result.err().unwrap(),
        ));
    }

    println!(
        "Generating {} for version {}",
        config.output_file_path.display(),
        config.version
    );

    generate_output(config)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_configuration_with_output_file() {
        let input_file_path = String::from("MAME 0.264 EXTRAs.zip");
        let output_file_path = String::from("extras.dat");
        let args = vec![input_file_path.clone(), output_file_path.clone()];

        let config_result = Config::build(&args);
        assert!(config_result.is_ok());
        let config = match config_result {
            Ok(config) => config,
            Err(e) => panic!("cannot read config: {}", e),
        };
        assert_eq!(
            input_file_path,
            config.input_file_path.display().to_string()
        );
        assert_eq!(
            output_file_path,
            config.output_file_path.display().to_string()
        );
        assert_eq!(0.264, config.version);
    }

    #[test]
    fn it_builds_configuration_without_output_file() {
        let input_file_path = String::from("MAME 0.264 EXTRAs.zip");
        let output_file_path = String::from("MAME 0.264 EXTRAs.dat");
        let args = vec![input_file_path.clone()];
        let config_result = Config::build(&args);
        assert!(config_result.is_ok());
        let config = match config_result {
            Ok(config) => config,
            Err(e) => panic!("cannot read config: {}", e),
        };
        assert_eq!(
            input_file_path,
            config.input_file_path.display().to_string()
        );
        assert_eq!(
            output_file_path,
            config.output_file_path.display().to_string()
        );
        assert_eq!(0.264, config.version);
    }

    #[test]
    fn it_builds_configuration_without_version() {
        let input_file_path = String::from("MAME EXTRAs.zip");
        let output_file_path = String::from("MAME EXTRAs.dat");
        let args = vec![input_file_path.clone()];
        let config_result = Config::build(&args);
        assert!(config_result.is_ok());
        let config = match config_result {
            Ok(config) => config,
            Err(e) => panic!("cannot read config: {}", e),
        };
        assert_eq!(
            input_file_path,
            config.input_file_path.display().to_string()
        );
        assert_eq!(
            output_file_path,
            config.output_file_path.display().to_string()
        );
        assert_eq!(0.01, config.version);
    }

    #[test]
    fn it_fails_to_build_configuration_without_input_file() {
        let args = vec![];
        let config_result = Config::build(&args);
        assert!(config_result.is_err());
        match config_result {
            Ok(_) => (),
            Err(e) => assert_eq!("missing input file", e.to_string()),
        };
    }

    #[test]
    fn it_returns_code_1_if_no_arguments() {
        let args = vec![];
        let code = real_main(&args);
        assert_eq!(1, code);
    }
}
