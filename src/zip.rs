use anyhow::anyhow;
use std::fs::{self, File};
use std::io::{BufReader, ErrorKind};
use std::path::Path;
use zip::ZipArchive;

use crate::files::FILES;

type Result<T> = anyhow::Result<T>;

/// Check if input file is accessible, is a valid Zip, and contains the expected entries.
pub fn check_input_file(input_file_path: &Path) -> Result<ZipArchive<BufReader<File>>> {
    let fname = Path::new(&input_file_path);

    // Check if input file exists and can be accessed
    if let Err(e) = fname.metadata() {
        match e.kind() {
            ErrorKind::NotFound => {
                return Err(anyhow!(
                    "the file `{}` does not exist",
                    input_file_path.display()
                ));
            }
            ErrorKind::PermissionDenied => {
                return Err(anyhow!(
                    "you have no permission to access file {0}",
                    input_file_path.display().to_string()
                ))
            }
            _ => {
                return Err(anyhow!(
                    "the file {} cannot be loaded",
                    input_file_path.display().to_string()
                ))
            }
        }
    }

    let file_result = fs::File::open(fname);
    let file = match file_result {
        Ok(file) => file,
        Err(_) => {
            return Err(anyhow!(
                "the file {} does not exist",
                input_file_path.display().to_string()
            ))
        }
    };
    let reader = BufReader::new(file);

    // Check if input file is a valid zip
    let archive_result = zip::ZipArchive::new(reader);
    let mut archive = match archive_result {
        Ok(archive) => archive,
        Err(_) => {
            return Err(anyhow!(
                "the file {} is not a valid Zip file",
                input_file_path.display().to_string()
            ))
        }
    };

    // Check if input ZIP file contains all expected files
    for name in FILES {
        if archive.by_name(name).is_err() {
            return Err(anyhow!("the entry {} is missing from input Zip file", name));
        }
    }

    Ok(archive)
}
