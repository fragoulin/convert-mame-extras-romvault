use std::env;
use std::process;

use convert_mame_extras_romvault::run;
use convert_mame_extras_romvault::Config;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        eprintln!(
            "Usage: {} <input file> <output file>",
            env::args().next().unwrap()
        );
        process::exit(1);
    });

    if let Err(e) = run(&config) {
        eprintln!("Error: {e}");
        process::exit(1);
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {elapsed:.2?}");

    process::exit(0);
}
