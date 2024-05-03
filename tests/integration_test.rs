mod common;

use std::fs;

use convert_mame_extras_romvault::real_main;
use temp_dir::TempDir;

use crate::common::compare_digests;

type Result<T> = anyhow::Result<T>;

#[test]
fn it_runs_with_1_argument() -> Result<()> {
    // Create arguments
    let input_file = String::from("tests/assets/MAME 0.264 EXTRAs.zip");
    let output_file = String::from("MAME 0.264 EXTRAs.dat");
    let expected_file = String::from("tests/assets/expected/MAME 0.264 EXTRAs.dat");
    let args = vec![input_file.clone()];

    // Run
    let code = real_main(&args);
    assert_eq!(0, code);

    // Compare files digests
    assert!(compare_digests(&output_file, &expected_file).unwrap());

    // Cleanup
    let result_remove = fs::remove_file(output_file);
    assert!(result_remove.is_ok());

    Ok(())
}

#[test]
fn it_runs_with_2_arguments_264() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let temp_dir_path = temp_dir.path();

    // Create arguments
    let input_file = String::from("tests/assets/MAME 0.264 EXTRAs.zip");
    let expected_file = String::from("tests/assets/expected/MAME 0.264 EXTRAs.dat");
    let output_file = temp_dir_path
        .join("extras264.dat")
        .to_string_lossy()
        .to_string();
    let args = vec![input_file.clone(), output_file.clone()];

    // Run
    let code = real_main(&args);
    assert_eq!(0, code);

    // Compare files digests
    assert!(compare_digests(&output_file, &expected_file).unwrap());

    Ok(())
}

#[test]
fn it_runs_with_2_arguments_262() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let temp_dir_path = temp_dir.path();

    // Create arguments
    let input_file = String::from("tests/assets/MAME 0.262 EXTRAs.zip");
    let expected_file = String::from("tests/assets/expected/MAME 0.262 EXTRAs.dat");
    let output_file = temp_dir_path
        .join("extras262.dat")
        .to_string_lossy()
        .to_string();
    let args = vec![input_file.clone(), output_file.clone()];

    // Run
    let code = real_main(&args);
    assert_eq!(0, code);

    // Compare files digests
    assert!(compare_digests(&output_file, &expected_file).unwrap());

    Ok(())
}
