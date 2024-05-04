//! Zip file handlers.

use anyhow::anyhow;
use std::fs::{self};
use std::io::{BufReader, ErrorKind};
use std::path::Path;

use crate::files::FILES;

/// Custom result with any context error.
type Result<T> = anyhow::Result<T>;

/// Check if input file is accessible, is a valid Zip, and contains the expected entries.
/// Expected entries are :
/// - all_non-zipped_content.dat
/// - artwork.dat
/// - samples.dat
///
/// # Errors
/// - File does not exists
/// - File not accessible (permission denied)
/// - File is not a valid Zip file
/// - Zip file doesn't contain expected entries
pub fn check_input_file(input_file_path: &Path) -> Result<()> {
    let fname = Path::new(&input_file_path);

    // Check if input file exists and can be accessed
    if let Err(err) = fname.metadata() {
        match err.kind() {
            ErrorKind::NotFound => {
                return Err(anyhow!(
                    "the file `{}` does not exist",
                    input_file_path.display()
                ));
            }
            ErrorKind::PermissionDenied => {
                return Err(anyhow!(
                    "you have no permission to access file `{0}`",
                    input_file_path.display().to_string()
                ))
            }
            _ => {
                return Err(anyhow!(
                    "the file `{}` cannot be loaded",
                    input_file_path.display().to_string()
                ))
            }
        }
    }

    let Ok(file) = fs::File::open(fname) else {
        return Err(anyhow!(
            "the file `{}` does not exist",
            input_file_path.display().to_string()
        ));
    };
    let reader = BufReader::new(file);

    // Check if input file is a valid zip
    let Ok(archive) = zip::ZipArchive::new(reader) else {
        return Err(anyhow!(
            "the file `{}` is not a valid Zip file",
            input_file_path.display().to_string()
        ));
    };

    // Check if input ZIP file contains all expected files
    let entries: Vec<&str> = archive.file_names().collect();
    if !FILES.iter().all(|item| entries.contains(item)) {
        return Err(anyhow!(
            "input Zip file must contains 3 files: {}",
            FILES.join(", "),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::env;

    use zip::write::SimpleFileOptions;

    use crate::files::{ALL_NON_ZIPPED_CONTENT, ARTWORK, SAMPLES};

    use super::*;

    #[test]
    fn it_should_handle_unexisting_file() {
        let file = Path::new("foo.txt");
        let result = check_input_file(file);
        assert!(result.is_err());
        match result {
            Ok(_) => (),
            Err(e) => assert_eq!("the file `foo.txt` does not exist", e.to_string()),
        };
    }

    #[test]
    fn it_should_handle_permission_denied_error() {
        let file = Path::new("/root/foo.txt");
        let result = check_input_file(file);
        assert!(result.is_err());
        match result {
            Ok(_) => (),
            Err(e) => assert_eq!(
                "you have no permission to access file `/root/foo.txt`",
                e.to_string()
            ),
        };
    }

    #[test]
    fn it_should_handle_invalid_zip() {
        let temp_dir = env::temp_dir();
        let file_path = "it_should_handle_invalid_zip.zip";
        let fname = temp_dir.join(file_path);
        let file_result = fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&fname);
        assert!(file_result.is_ok());
        let result = check_input_file(&fname);
        assert!(result.is_err());
        match result {
            Ok(_) => (),
            Err(e) => assert_eq!(
                format!("the file `{}` is not a valid Zip file", fname.display()),
                e.to_string()
            ),
        };
        let remove_result = fs::remove_file(fname);
        assert!(remove_result.is_ok());
    }

    #[test]
    fn it_should_handle_missing_all_content_file_in_zip() {
        it_should_handle_missing_file_in_zip(
            "it_should_handle_missing_all_content_file_in_zip.zip",
            ARTWORK,
            SAMPLES,
        );
    }

    #[test]
    fn it_should_handle_missing_artwork_file_in_zip() {
        it_should_handle_missing_file_in_zip(
            "it_should_handle_missing_artwork_file_in_zip.zip",
            ALL_NON_ZIPPED_CONTENT,
            SAMPLES,
        );
    }

    #[test]
    fn it_should_handle_missing_samples_file_in_zip() {
        it_should_handle_missing_file_in_zip(
            "it_should_handle_missing_samples_file_in_zip.zip",
            ALL_NON_ZIPPED_CONTENT,
            ARTWORK,
        );
    }

    fn it_should_handle_missing_file_in_zip(file_path: &str, file1: &str, file2: &str) {
        let temp_dir = env::temp_dir();
        let fname = temp_dir.join(file_path);
        let file_result = fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&fname);
        assert!(file_result.is_ok());

        let mut zip = zip::ZipWriter::new(file_result.unwrap());
        let options = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Stored)
            .unix_permissions(0o755);
        let start_file_result = zip.start_file(file1, options);
        assert!(start_file_result.is_ok());
        let start_file_result = zip.start_file(file2, options);
        assert!(start_file_result.is_ok());
        let zip_finish_result = zip.finish();
        assert!(zip_finish_result.is_ok());

        let result = check_input_file(&fname);
        assert!(result.is_err());
        match result {
            Ok(_) => (),
            Err(e) => assert_eq!(
                format!("input Zip file must contains 3 files: all_non-zipped_content.dat, artwork.dat, samples.dat"),
                e.to_string()
            ),
        };

        let remove_result = fs::remove_file(fname);
        assert!(remove_result.is_ok());
    }
}
