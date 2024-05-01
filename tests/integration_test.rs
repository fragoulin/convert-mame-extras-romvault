mod common;

use std::fs;

use convert_mame_extras_romvault::real_main;

use crate::common::compare_digests;

type Result<T> = anyhow::Result<T>;

#[test]
fn it_runs_with_1_argument() -> Result<()> {
    // Create arguments
    let input_file = String::from("tests/assets/MAME 0.264 EXTRAs.zip");
    let output_file = String::from("tests/assets/MAME 0.264 EXTRAs.dat");
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
    // Create arguments
    let input_file = String::from("tests/assets/MAME 0.264 EXTRAs.zip");
    let expected_file = String::from("tests/assets/expected/MAME 0.264 EXTRAs.dat");
    let output_file = String::from("extras264.dat");
    let args = vec![input_file.clone(), output_file.clone()];

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
fn it_runs_with_2_arguments_262() -> Result<()> {
    // Create arguments
    let input_file = String::from("tests/assets/MAME 0.262 EXTRAs.zip");
    let expected_file = String::from("tests/assets/expected/MAME 0.262 EXTRAs.dat");
    let output_file = String::from("extras262.dat");
    let args = vec![input_file.clone(), output_file.clone()];

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
