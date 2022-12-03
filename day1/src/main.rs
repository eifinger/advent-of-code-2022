use std::{env, process};

use day1::{run};

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Err(e) = run(&args[1]) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
