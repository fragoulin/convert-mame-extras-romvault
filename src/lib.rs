pub mod dat;
pub mod files;
pub mod zip;

use crate::{dat::generate_output, files::cleanup_temp_dir, zip::check_input_file};
use anyhow::anyhow;
use files::extract_version;
use std::{env::temp_dir, path::PathBuf};

type Result<T> = anyhow::Result<T>;

pub struct Config {
    input_file_path: PathBuf,
    output_file_path: PathBuf,
    version: f32,
    pub temp_dir_path: PathBuf,
}

impl Config {
    /// # Errors
    ///
    /// Will return `Err` if input file argument is missing.
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config> {
        args.next(); // Skip executable name

        let input_file_path = match args.next() {
            Some(input_file_path) => PathBuf::from(&input_file_path),
            None => return Err(anyhow!("missing input file")),
        };
        let output_file_path = match args.next() {
            Some(output_file_path) => PathBuf::from(&output_file_path),
            None => {
                let mut output_file_path = PathBuf::from(&input_file_path);
                output_file_path.set_extension("dat");
                output_file_path
            }
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
        &config.input_file_path.display(),
        &config.temp_dir_path.display()
    );

    if archive.extract(&config.temp_dir_path).is_err() {
        return Err(anyhow!(
            "failed to extract zip file {} into directory {}",
            String::from(&config.input_file_path.display().to_string()),
            &config.temp_dir_path.display()
        ));
    }

    println!(
        "Generating {} for version {}",
        config.output_file_path.display(),
        config.version
    );

    generate_output(&config.output_file_path, config.version)?;

    println!("Cleanup files in {}", config.temp_dir_path.display());

    cleanup_temp_dir(&config.temp_dir_path)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn it_builds_configuration_with_output_file() {
        let input_file_path = String::from("MAME 0.264 EXTRAs.zip");
        let output_file_path = String::from("extras.dat");
        let args = [
            String::from("convert-mame-extras-romvault"),
            input_file_path.clone(),
            output_file_path.clone(),
        ]
        .into_iter();
        let config_result = Config::build(args);
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
        assert_eq!(env::temp_dir(), config.temp_dir_path);
    }

    #[test]
    fn it_builds_configuration_without_output_file() {
        let input_file_path = String::from("MAME 0.264 EXTRAs.zip");
        let output_file_path = String::from("MAME 0.264 EXTRAs.dat");
        let args = [
            String::from("convert-mame-extras-romvault"),
            input_file_path.clone(),
        ]
        .into_iter();
        let config_result = Config::build(args);
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
        assert_eq!(env::temp_dir(), config.temp_dir_path);
    }

    #[test]
    fn it_builds_configuration_without_version() {
        let input_file_path = String::from("MAME EXTRAs.zip");
        let output_file_path = String::from("MAME EXTRAs.dat");
        let args = [
            String::from("convert-mame-extras-romvault"),
            input_file_path.clone(),
        ]
        .into_iter();
        let config_result = Config::build(args);
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
        assert_eq!(env::temp_dir(), config.temp_dir_path);
    }

    #[test]
    fn it_fails_to_build_configuration_without_input_file() {
        let args = [String::from("convert-mame-extras-romvault")].into_iter();
        let config_result = Config::build(args);
        assert!(config_result.is_err());
        let _ = match config_result {
            Ok(_) => (),
            Err(e) => assert_eq!("missing input file", e.to_string()),
        };
    }
}
