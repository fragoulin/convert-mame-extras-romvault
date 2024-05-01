pub mod dat;
pub mod files;
pub mod zip;

use crate::{dat::generate_output, zip::check_input_file};
use anyhow::anyhow;
use files::extract_version;
use std::{path::PathBuf, time::Instant};
use temp_dir::TempDir;

type Result<T> = anyhow::Result<T>;

struct Config {
    input_file_path: PathBuf,
    output_file_path: PathBuf,
    version: f32,
    pub temp_dir_path: TempDir,
}

impl Config {
    /// # Errors
    ///
    /// Will return `Err` if input file argument is missing.
    fn build(args: &[String]) -> Result<Config> {
        if args.len() == 0 {
            return Err(anyhow!("missing input file"));
        }

        let input_file_path = PathBuf::from(&args[0]);
        let mut output_file_path: PathBuf;

        if args.len() == 1 {
            // No output file specified
            output_file_path = PathBuf::from(&input_file_path);
            output_file_path.set_extension("dat");
        } else {
            output_file_path = PathBuf::from(&args[1]);
        }

        let version = extract_version(&input_file_path).unwrap_or(0.01);

        let temp_dir_path = TempDir::new().unwrap();

        Ok(Config {
            input_file_path,
            output_file_path,
            version,
            temp_dir_path,
        })
    }
}

pub fn real_main(args: &[String]) -> i32 {
    let now = Instant::now();

    let config_result = Config::build(args);
    if config_result.is_err() {
        eprintln!(
            "Problem parsing arguments: {}",
            config_result.err().unwrap()
        );
        print_usage();
        return 1;
    }

    let config = config_result.unwrap();
    let run_result = run(&config);
    if run_result.is_err() {
        eprintln!("Error: {}", run_result.err().unwrap());
        return 1;
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {elapsed:.2?}");

    0
}

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

    if archive.extract(&config.temp_dir_path.path()).is_err() {
        return Err(anyhow!(
            "failed to extract zip file {} into directory {}",
            String::from(&config.input_file_path.display().to_string()),
            &config.temp_dir_path.path().display()
        ));
    }

    println!(
        "Generating {} for version {}",
        config.output_file_path.display(),
        config.version
    );

    generate_output(
        &config.output_file_path,
        config.version,
        config.temp_dir_path.path(),
    )?;

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
        let _ = match config_result {
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
