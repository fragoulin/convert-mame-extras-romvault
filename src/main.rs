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
        eprintln!(
            "Usage: {} <inputfile> <outputfile>",
            env::args().next().unwrap()
        );
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
