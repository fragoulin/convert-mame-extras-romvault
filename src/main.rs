//! # Convert MAME Extras Romvault
//!
//! This crate can be used to convert MAME Extras Zip file
//! to a compatible format to use with Romvault.

use std::process;

use convert_mame_extras_romvault::real_main;

/// Begin main process.
/// At least one argument is mandatory : MAME extras Zip file
/// Optional second argument is the output file. If not specified,
/// the input file name is used (`MAME 0.264 Extras.zip` => `MAME 0.264 Extras.dat`).
///
/// # Examples
///
/// ```
/// convert-mame-extras-romvault MAME\ 0.264\ EXTRAs.zip
/// ```
///
/// ```
/// convert-mame-extras-romvault MAME\ 0.264\ EXTRAs.zip extras.dat
/// ```
///
fn main() {
    let code = real_main();
    process::exit(code.into());
}
