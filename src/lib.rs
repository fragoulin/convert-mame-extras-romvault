//! # Convert MAME Extras Romvault
//!
//! This crate can be used to convert MAME Extras Zip file
//! to a compatible format to use with Romvault.

pub mod dat;
pub mod files;
pub mod zip;

use crate::dat::generate_output;
use anyhow::anyhow;
use files::extract_version;
use std::{path::PathBuf, time::Instant};

/// Custom result with any context error.
type Result<T> = anyhow::Result<T>;

/// Main configuration to hold various parameters:
/// - Input from command line arguments
/// - Version computed from input file name
/// - Temporary directory path
/// - XML readers for the three dat files
pub struct Config {
    /// Zip file used for input.
    input_file_path: PathBuf,
    /// Generated dat will be written into this output file.
    output_file_path: PathBuf,
    /// Version extracted from input Zip file name. Will be used for dat generation.
    version: f32,
}

/// Input and output files from command line arguments.
struct Input {
    /// Zip file used for input.
    input_file_path: PathBuf,
    /// Generated dat will be written into this output file.
    output_file_path: PathBuf,
}

impl Config {
    /// Build configuration according to specified input
    fn build(input: Input) -> Self {
        let file_name = input.input_file_path.display().to_string();
        let version = extract_version(file_name.as_str()).unwrap_or(0.01);

        Self {
            input_file_path: input.input_file_path,
            output_file_path: input.output_file_path,
            version,
        }
    }
}

/// Parse arguments, build XML readers, and tries to generate output dat file.
///
/// Returns 0 if no error occurred.
/// Returns 1 in case of error.
#[must_use]
pub fn real_main(args: &[String]) -> i32 {
    let now = Instant::now();

    let input = match parse_args(args) {
        Ok(input) => input,
        Err(err) => {
            eprintln!("Problem parsing arguments: {err}",);
            print_usage();
            return 1;
        }
    };

    let config = Config::build(input);

    if let Err(err) = run(&config) {
        eprintln!("Error: {err}");
        return 1;
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {elapsed:.2?}");

    0
}

/// Parse command line arguments and build Input struct.
fn parse_args(args: &[String]) -> Result<Input> {
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

    let result = Input {
        input_file_path,
        output_file_path,
    };

    Ok(result)
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
    // Test ok -> remove temp dir, remove extract, put readers for all 3 dats in config
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

        let input_result = parse_args(&args);
        assert!(input_result.is_ok());
        let config = Config::build(input_result.unwrap());

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

        let input_result = parse_args(&args);
        assert!(input_result.is_ok());
        let config = Config::build(input_result.unwrap());

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

        let input_result = parse_args(&args);
        assert!(input_result.is_ok());
        let config = Config::build(input_result.unwrap());

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

        let input_result = parse_args(&args);
        assert!(input_result.is_err());
        match input_result {
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
