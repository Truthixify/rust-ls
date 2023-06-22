use std::env::args;
use std::process;

use ls::{Config, run};

fn main() {
    let args: Vec<String> = args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let path = config.path;

    print!("\nListing directory... \n\n");

    println!("Type \t \t Cre \t \t Mod \t \t Acc \t \t Name");

    run(path.clone());
}