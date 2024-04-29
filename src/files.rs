pub const ALL_NON_ZIPPED_CONTENT: &str = "all_non-zipped_content.dat";
pub const ARTWORK: &str = "artwork.dat";
pub const SAMPLES: &str = "samples.dat";
pub const FILES: [&str; 3] = [ALL_NON_ZIPPED_CONTENT, ARTWORK, SAMPLES];

use std::{error::Error, fs, path::PathBuf};

use regex::Regex;

/// Tries to extract MAME version from specified input file path.
pub fn extract_version(input_file_path: &str) -> Option<f32> {
    let re = Regex::new(r"MAME (?<version>\d\.\d+) EXTRAs\.zip$").unwrap();
    let caps = re.captures(input_file_path)?;
    let version = caps["version"].parse::<f32>();
    match version {
        Ok(version) => Some(version),
        Err(_) => None,
    }
}

/// Tries to cleanup files extracted from input ZIP file in temporary directory.
///
/// # Errors
///
/// Will return `Err` if files cannot be removed.
pub fn cleanup_temp_dir(directory: &PathBuf) -> Result<(), Box<dyn Error>> {
    for file in FILES {
        let path = PathBuf::from(&directory).join(file);
        fs::remove_file(path)?;
    }

    Ok(())
}
