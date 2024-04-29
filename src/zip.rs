use std::error::Error;
use std::fmt;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;
use zip::ZipArchive;

use crate::files::FILES;

#[derive(Debug)]
struct InvalidZipContentError(String);

impl fmt::Display for InvalidZipContentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot find file '{}' in Zip archive", self.0)
    }
}

impl Error for InvalidZipContentError {}

/// Check if input file is accessible, is a valid Zip, and contains the expected entries.
/// # Errors
///
/// Will return `Err` for one of the following cases:
/// - `input_file_path` doesn't exist
/// - `input_file_path` cannot be accessed,
/// - `input_file_path` is not a valid Zip archive
/// - archive doesn't contain expected files
pub fn check_input_file(
    input_file_path: &String,
) -> Result<ZipArchive<BufReader<File>>, Box<dyn Error>> {
    let fname = Path::new(&input_file_path);

    // Check if input file exists and can be accessed
    fname.metadata()?;

    let file = fs::File::open(fname)?;
    let reader = BufReader::new(file);

    // Check if input file is a valid zip
    let mut archive = zip::ZipArchive::new(reader)?;

    // Check if input ZIP file contains all expected files
    for name in FILES {
        if archive.by_name(name).is_err() {
            // Use custom error in order to show missing file in message
            return Err(Box::new(InvalidZipContentError(name.into())));
        }
    }

    Ok(archive)
}
