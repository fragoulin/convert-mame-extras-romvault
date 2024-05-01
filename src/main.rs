use std::env;
use std::process;

use convert_mame_extras_romvault::files::cleanup_temp_dir;
use convert_mame_extras_romvault::run;
use convert_mame_extras_romvault::Config;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        print_usage();
        process::exit(1);
    });

    if let Err(e) = run(&config) {
        eprintln!("Error: {e}");
        let _ = cleanup_temp_dir(&config.temp_dir_path);
        process::exit(1);
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {elapsed:.2?}");

    process::exit(0);
}

fn print_usage() {
    let indent = 7;
    eprintln!("Usage: convert-mame-extras-romvault <inputfile> <outputfile>");
    eprintln!("{:indent$}<inputfile> is mandatory and must be a valid Zip file (e.g. `MAME 0.264 EXTRAs.zip`)", "");
    eprintln!("{:indent$}<outputfile> is optional. If not specified, the name of the input file will be used (e.g. `MAME 0.264 EXTRAs.dat`)", "");
}
