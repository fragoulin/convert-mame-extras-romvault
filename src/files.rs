//! Operations on files.

/// File all_non-zipped_content.dat which should be in Zip input file.
pub const ALL_NON_ZIPPED_CONTENT: &str = "all_non-zipped_content.dat";
/// File artwork.dat which should be in Zip input file.
pub const ARTWORK: &str = "artwork.dat";
/// File samples.dat which should be in Zip input file.
pub const SAMPLES: &str = "samples.dat";
/// All dat files in a convenient array.
pub const FILES: [&str; 3] = [ALL_NON_ZIPPED_CONTENT, ARTWORK, SAMPLES];

use regex::RegexBuilder;

/// Tries to extract MAME version from specified input file path.
///
/// # Examples
///
/// ```
/// let version = convert_mame_extras_romvault::files::extract_version("dats/MAME 0.262 Extras.zip");
/// assert_eq!(Some(0.262), version);
/// ```
///
/// ```
/// let version = convert_mame_extras_romvault::files::extract_version("dats/MAME 0.264 Extras.ZIP");
/// assert_eq!(Some(0.264), version);
/// ```
///
/// ```
/// let version = convert_mame_extras_romvault::files::extract_version("dats/Extras.zip");
/// assert_eq!(None, version);
/// ```
#[must_use]
pub fn extract_version(file_name: &str) -> Option<f32> {
    let Ok(re) = RegexBuilder::new(r"MAME (?<version>\d\.\d+) EXTRAs\.zip$")
        .case_insensitive(true)
        .build()
    else {
        return None; // Should never happen
    };
    let caps = re.captures(file_name)?;
    let Ok(version) = caps["version"].parse::<f32>() else {
        return None;
    };

    Some(version)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_extracts_version_0264() {
        let input_file_path = "MAME 0.264 EXTRAs.zip";
        let version = extract_version(&input_file_path);
        assert!(version.is_some());
        assert_eq!(0.264, version.unwrap());
    }

    #[test]
    fn it_extracts_version_0264_if_lowercase() {
        let input_file_path = "mame 0.264 extras.zip";
        let version = extract_version(&input_file_path);
        assert!(version.is_some());
        assert_eq!(0.264, version.unwrap());
    }

    #[test]
    fn it_extracts_version_10() {
        let input_file_path = "MAME 1.0 EXTRAs.zip";
        let version = extract_version(&input_file_path);
        assert!(version.is_some());
        assert_eq!(1.0, version.unwrap());
    }

    #[test]
    fn it_handles_file_without_version() {
        let input_file_path = "MAME EXTRAs.zip";
        let version = extract_version(&input_file_path);
        assert!(version.is_none());
    }

    #[test]
    fn it_handles_empty_file() {
        let input_file_path = "";
        let version = extract_version(&input_file_path);
        assert!(version.is_none());
    }
}
