//! # Convert MAME Extras Romvault
//!
//! This crate can be used to convert MAME Extras Zip file
//! to a compatible format to use with Romvault.

pub mod dat;
pub mod files;
pub mod zip;

use crate::{dat::generate_output, zip::check_input_file};
use clap::Parser;
use files::extract_version;
use std::{path::PathBuf, time::Instant};

/// Main configuration to hold various parameters:
/// - Input file from command line arguments
/// - Output file from command line arguments or generated from input file name
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
        let input_file_name = args.input_file.file_name().unwrap();
        let version = extract_version(input_file_name.to_str().unwrap());

        let input_file_path = PathBuf::from(&args.input_file);
        let mut output_file_path: PathBuf;

        if args.output_file.is_none() {
            // Compute output file name from input file name
            output_file_path = PathBuf::from(input_file_name);
            output_file_path.set_extension("dat");
        } else {
            output_file_path = PathBuf::from(args.output_file.as_ref().unwrap());
        }

        Self {
            input_file_path,
            output_file_path,
            version,
        }
    }
}

/// Convert MAME Extras to Romvault format.
#[derive(Parser)]
struct Args {
    /// Input Zip file containing MAME Extras dats (all_non-zipped_content.dat, artwork.dat and samples.dat).
    input_file: PathBuf,
    /// Optional output file compatible with RomVault. If not specified, the input Zip file name is used, with a .dat extension.
    output_file: Option<PathBuf>,
}

/// Parse arguments, build application configuration, and tries to generate output dat file.
///
/// Returns 0 if no error occurred.
/// Returns 1 in case of error.
#[must_use]
pub fn real_main() -> i8 {
    let now = Instant::now();

    // Parse arguments
    let args = Args::parse();

    if let Err(err) = check_input_file(args.input_file.as_path()) {
        eprintln!("Error: {err}");
        return 1;
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
