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

    let file_result = fs::File::open(fname);
    let file = match file_result {
        Ok(file) => file,
        Err(_) => {
            return Err(anyhow!(
                "the file `{}` does not exist",
                input_file_path.display().to_string()
            ))
        }
    };
    let reader = BufReader::new(file);

    // Check if input file is a valid zip
    let archive_result = zip::ZipArchive::new(reader);
    let archive = match archive_result {
        Ok(archive) => archive,
        Err(_) => {
            return Err(anyhow!(
                "the file `{}` is not a valid Zip file",
                input_file_path.display().to_string()
            ))
        }
    };

    // Check if input ZIP file contains all expected files
    let entries: Vec<&str> = archive.file_names().collect();
    for name in FILES {
        if !entries.contains(&name) {
            return Err(anyhow!(
                "the entry `{}` is missing from input Zip file",
                name
            ));
        }
    }

    Ok(archive)
}

#[cfg(test)]
mod tests {
    use std::env;

    use zip::write::SimpleFileOptions;

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
            FILES[0],
            "it_should_handle_missing_all_content_file_in_zip.zip",
            FILES[1],
            FILES[2],
        );
    }

    #[test]
    fn it_should_handle_missing_artwork_file_in_zip() {
        it_should_handle_missing_file_in_zip(
            FILES[1],
            "it_should_handle_missing_artwork_file_in_zip.zip",
            FILES[0],
            FILES[2],
        );
    }

    #[test]
    fn it_should_handle_missing_samples_file_in_zip() {
        it_should_handle_missing_file_in_zip(
            FILES[2],
            "it_should_handle_missing_samples_file_in_zip.zip",
            FILES[0],
            FILES[1],
        );
    }

    fn it_should_handle_missing_file_in_zip(
        entry: &str,
        file_path: &str,
        file1: &str,
        file2: &str,
    ) {
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
                format!("the entry `{}` is missing from input Zip file", entry),
                e.to_string()
            ),
        };

        let remove_result = fs::remove_file(fname);
        assert!(remove_result.is_ok());
    }
}
