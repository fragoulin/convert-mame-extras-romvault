pub const ALL_NON_ZIPPED_CONTENT: &str = "all_non-zipped_content.dat";
pub const ARTWORK: &str = "artwork.dat";
pub const SAMPLES: &str = "samples.dat";
pub const FILES: [&str; 3] = [ALL_NON_ZIPPED_CONTENT, ARTWORK, SAMPLES];

use std::path::Path;

use regex::RegexBuilder;

/// Tries to extract MAME version from specified input file path.
#[must_use]
pub fn extract_version(input_file_path: &Path) -> Option<f32> {
    let Ok(re) = RegexBuilder::new(r"MAME (?<version>\d\.\d+) EXTRAs\.zip$")
        .case_insensitive(true)
        .build()
    else {
        return None;
    };
    let file_path = input_file_path.display().to_string();
    let caps = re.captures(file_path.as_str())?;
    let Ok(version) = caps["version"].parse::<f32>() else {
        return None;
    };

    Some(version)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn it_extracts_version_0264() {
        let input_file_path = "MAME 0.264 EXTRAs.zip";
        let version = extract_version(&PathBuf::from(&input_file_path));
        assert!(version.is_some());
        assert_eq!(0.264, version.unwrap());
    }

    #[test]
    fn it_extracts_version_0264_if_lowercase() {
        let input_file_path = "mame 0.264 extras.zip";
        let version = extract_version(&PathBuf::from(&input_file_path));
        assert!(version.is_some());
        assert_eq!(0.264, version.unwrap());
    }

    #[test]
    fn it_extracts_version_10() {
        let input_file_path = "MAME 1.0 EXTRAs.zip";
        let version = extract_version(&PathBuf::from(&input_file_path));
        assert!(version.is_some());
        assert_eq!(1.0, version.unwrap());
    }

    #[test]
    fn it_handles_file_without_version() {
        let input_file_path = "MAME EXTRAs.zip";
        let version = extract_version(&PathBuf::from(&input_file_path));
        assert!(version.is_none());
    }

    #[test]
    fn it_handles_empty_file() {
        let input_file_path = "";
        let version = extract_version(&PathBuf::from(&input_file_path));
        assert!(version.is_none());
    }
}
