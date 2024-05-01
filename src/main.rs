use std::{env, process};

use convert_mame_extras_romvault::real_main;

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let code = real_main(&args);
    process::exit(code);
}
