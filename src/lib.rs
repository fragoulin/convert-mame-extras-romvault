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
pub struct Config {
    /// Zip file used for input.
    input_file_path: PathBuf,
    /// Generated dat will be written into this output file.
    output_file_path: PathBuf,
    /// Version extracted from input Zip file name. Will be used for dat generation.
    version: Option<f32>,
}

impl Config {
    /// Build configuration according to specified command line arguments
    fn build(args: &Args) -> Self {
        let version = extract_version(args.input_file);

        let input_file_path = PathBuf::from(args.input_file);
        let mut output_file_path: PathBuf;

        if args.output_file.is_none() {
            // Compute output file name from input file name
            let input_file_name = input_file_path.file_name().unwrap();
            output_file_path = PathBuf::from(input_file_name);
            output_file_path.set_extension("dat");
        } else {
            output_file_path = PathBuf::from(args.output_file.unwrap());
        }

        Self {
            input_file_path,
            output_file_path,
            version,
        }
    }
}

/// Arguments parsed from command line.
struct Args<'a> {
    /// Mandatory input file retrieved from command line.
    input_file: &'a str,
    /// Optional output file retrieved from command line.
    output_file: Option<&'a str>,
}

/// Parse arguments, build application configuration, and tries to generate output dat file.
///
/// Returns 0 if no error occurred.
/// Returns 1 in case of error.
#[must_use]
pub fn real_main(args: &[String]) -> i8 {
    let now = Instant::now();

    // Parse arguments
    let args = match parse_args(args) {
        Ok(args) => args,
        Err(err) => {
            eprintln!("Problem parsing arguments: {err}",);
            print_usage();
            return 1;
        }
    };

    // Build configuration
    let config = Config::build(&args);
    println!(
        "Generating {} for version {}",
        config.output_file_path.display(),
        config.version.unwrap_or_default(),
    );

    // Generate output dat file
    if let Err(err) = generate_output(&config) {
        eprintln!("Error: {err}");
        return 1;
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {elapsed:.2?}");

    0
}

/// Parse command line arguments and return a struct containing input and output file paths.
fn parse_args(args: &[String]) -> Result<Args> {
    if args.is_empty() {
        return Err(anyhow!("missing input file"));
    }

    let input_file = args[0].as_str();
    let output_file = if args.len() > 1 {
        Some(args[1].as_str())
    } else {
        None
    };
    let result = Args {
        input_file,
        output_file,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_configuration_with_output_file() {
        let input_file_path = String::from("MAME 0.264 EXTRAs.zip");
        let output_file_path = String::from("extras.dat");
        let args = vec![input_file_path.clone(), output_file_path.clone()];

        let args_result = parse_args(&args);
        assert!(args_result.is_ok());
        let args = args_result.unwrap();
        let config = Config::build(&args);

        assert_eq!(
            input_file_path,
            config.input_file_path.display().to_string()
        );
        assert_eq!(
            output_file_path,
            config.output_file_path.display().to_string()
        );
        assert_eq!(0.264, config.version.unwrap());
    }

    #[test]
    fn it_builds_configuration_without_output_file() {
        let input_file_path = String::from("MAME 0.264 EXTRAs.zip");
        let output_file_path = String::from("MAME 0.264 EXTRAs.dat");
        let args = vec![input_file_path.clone()];

        let args_result = parse_args(&args);
        assert!(args_result.is_ok());
        let args = args_result.unwrap();
        let config = Config::build(&args);

        assert_eq!(
            input_file_path,
            config.input_file_path.display().to_string()
        );
        assert_eq!(
            output_file_path,
            config.output_file_path.display().to_string()
        );
        assert_eq!(0.264, config.version.unwrap());
    }

    #[test]
    fn it_builds_configuration_without_version() {
        let input_file_path = String::from("MAME EXTRAs.zip");
        let output_file_path = String::from("MAME EXTRAs.dat");
        let args = vec![input_file_path.clone()];

        let args_result = parse_args(&args);
        assert!(args_result.is_ok());
        let args = args_result.unwrap();
        let config = Config::build(&args);

        assert_eq!(
            input_file_path,
            config.input_file_path.display().to_string()
        );
        assert_eq!(
            output_file_path,
            config.output_file_path.display().to_string()
        );
        assert_eq!(true, config.version.is_none());
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
