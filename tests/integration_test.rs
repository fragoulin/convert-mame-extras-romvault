//! Integration tests.

mod common;

use assert_cmd::prelude::*; // Add methods on commands
use std::process::Command;
use std::{env, fs}; // Run programs

use crate::common::compare_digests;

type Result<T> = anyhow::Result<T>;

#[test]
fn it_fails_with_0_argument() -> Result<()> {
    // Create arguments
    let mut cmd = Command::cargo_bin("convert-mame-extras-romvault")?;
    let status = cmd.status().expect("Failure");

    assert_eq!(2, status.code().unwrap());

    Ok(())
}

#[test]
fn it_fails_with_invalid_input() -> Result<()> {
    // Create arguments
    let input_file = String::from("tests/assets/expected/MAME 0.264 EXTRAs.dat");
    let mut cmd = Command::cargo_bin("convert-mame-extras-romvault")?;
    let status = cmd.arg(input_file).status().expect("Failure");

    assert_eq!(1, status.code().unwrap());
    let output = cmd.output().unwrap().stderr;
    assert_eq!(
        String::from_utf8(output).unwrap(),
        "Error: the file `tests/assets/expected/MAME 0.264 EXTRAs.dat` is not a valid Zip file\n"
    );

    Ok(())
}

#[test]
fn it_runs_with_1_argument() -> Result<()> {
    // Create arguments
    let input_file = String::from("tests/assets/MAME 0.264 EXTRAs.zip");
    let output_file = String::from("MAME 0.264 EXTRAs.dat");
    let expected_file = String::from("tests/assets/expected/MAME 0.264 EXTRAs.dat");
    let mut cmd = Command::cargo_bin("convert-mame-extras-romvault")?;
    let status = cmd.arg(input_file).status().expect("Failure");

    assert!(status.success());

    // Compare files digests
    assert!(compare_digests(&output_file, &expected_file).unwrap());

    // Cleanup
    let result_remove = fs::remove_file(output_file);
    assert!(result_remove.is_ok());

    Ok(())
}

fn it_runs_with_2_arguments(input_file: String, expected_file: String, output_file_name: &str) -> Result<()> {
    let temp_dir_path = env::temp_dir();

    // Create arguments
    let output_file_path = temp_dir_path.join(output_file_name);
    let output_file = output_file_path.to_string_lossy().to_string();

    let mut cmd = Command::cargo_bin("convert-mame-extras-romvault")?;
    let status = cmd
        .arg(input_file)
        .arg(output_file.clone())
        .status()
        .expect("Failure");

    assert!(status.success());

    // Compare files digests
    assert!(compare_digests(&output_file, &expected_file).unwrap());

    assert!(fs::remove_file(output_file_path).is_ok());

    Ok(())
}

#[test]
fn it_runs_with_2_arguments_262() -> Result<()> {
    let input_file = String::from("tests/assets/MAME 0.262 EXTRAs.zip");
    let expected_file = String::from("tests/assets/expected/MAME 0.262 EXTRAs.dat");
    it_runs_with_2_arguments(input_file, expected_file, "extras262.dat")
}

#[test]
fn it_runs_with_2_arguments_264() -> Result<()> {
    let input_file = String::from("tests/assets/MAME 0.264 EXTRAs.zip");
    let expected_file = String::from("tests/assets/expected/MAME 0.264 EXTRAs.dat");
    it_runs_with_2_arguments(input_file, expected_file, "extras264.dat")
}

#[test]
fn it_runs_with_2_arguments_266() -> Result<()> {
    let input_file = String::from("tests/assets/MAME 0.266 EXTRAs.zip");
    let expected_file = String::from("tests/assets/expected/MAME 0.266 EXTRAs.dat");
    it_runs_with_2_arguments(input_file, expected_file, "extras266.dat")
}

#[test]
fn it_runs_with_2_arguments_269() -> Result<()> {
    let input_file = String::from("tests/assets/MAME 0.269 EXTRAs.zip");
    let expected_file = String::from("tests/assets/expected/MAME 0.269 EXTRAs.dat");
    it_runs_with_2_arguments(input_file, expected_file, "extras269.dat")
}

#[test]
fn it_runs_with_2_arguments_270() -> Result<()> {
    let input_file = String::from("tests/assets/MAME 0.270 EXTRAs.zip");
    let expected_file = String::from("tests/assets/expected/MAME 0.270 EXTRAs.dat");
    it_runs_with_2_arguments(input_file, expected_file, "extras270.dat")
}

#[test]
fn it_runs_with_2_arguments_272() -> Result<()> {
    let input_file = String::from("tests/assets/MAME 0.272 EXTRAs.zip");
    let expected_file = String::from("tests/assets/expected/MAME 0.272 EXTRAs.dat");
    it_runs_with_2_arguments(input_file, expected_file, "extras272.dat")
}

#[test]
fn it_runs_with_2_arguments_276() -> Result<()> {
    let input_file = String::from("tests/assets/MAME 0.276 EXTRAs.zip");
    let expected_file = String::from("tests/assets/expected/MAME 0.276 EXTRAs.dat");
    it_runs_with_2_arguments(input_file, expected_file, "extras276.dat")
}
