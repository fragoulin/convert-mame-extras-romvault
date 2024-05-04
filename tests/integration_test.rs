mod common;

use std::{env, fs};

use convert_mame_extras_romvault::real_main;

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
    let temp_dir_path = env::temp_dir();

    // Create arguments
    let input_file = String::from("tests/assets/MAME 0.264 EXTRAs.zip");
    let expected_file = String::from("tests/assets/expected/MAME 0.264 EXTRAs.dat");
    let output_file_name = "extras264.dat";
    let output_file_path = temp_dir_path.join(output_file_name);
    let output_file = output_file_path.to_string_lossy().to_string();
    let args = vec![input_file.clone(), output_file.clone()];

    // Run
    let code = real_main(&args);
    assert_eq!(0, code);

    // Compare files digests
    assert!(compare_digests(&output_file, &expected_file).unwrap());

    assert!(fs::remove_file(output_file_path).is_ok());

    Ok(())
}

#[test]
fn it_runs_with_2_arguments_262() -> Result<()> {
    let temp_dir_path = env::temp_dir();

    // Create arguments
    let input_file = String::from("tests/assets/MAME 0.262 EXTRAs.zip");
    let expected_file = String::from("tests/assets/expected/MAME 0.262 EXTRAs.dat");
    let output_file_name = "extras262.dat";
    let output_file_path = temp_dir_path.join(output_file_name);
    let output_file = output_file_path.to_string_lossy().to_string();
    let args = vec![input_file.clone(), output_file.clone()];

    // Run
    let code = real_main(&args);
    assert_eq!(0, code);

    // Compare files digests
    assert!(compare_digests(&output_file, &expected_file).unwrap());

    assert!(fs::remove_file(output_file_path).is_ok());

    Ok(())
}
