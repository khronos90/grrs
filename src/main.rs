#![allow(unused)]

use clap::Parser;
use std::io::prelude::*;
/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");
    // let args = Cli {
    //     pattern,
    //     path: std::path::PathBuf::from(path),
    // };

    let cli_args = Cli::parse();

    let file = std::fs::File::open(&cli_args.path).expect("Couldn't open file");
    let mut reader = std::io::BufReader::new(file);

    let mut line = String::new();

    fn matches(val: String, args: &Cli) -> () {
        if val.contains(&args.pattern) {
            println!("{}", val)
        }
    }

    for line in reader.lines() {
        match line {
            Ok(res) => matches(res, &cli_args),
            Err(err) => println!("{}", err),
        };
    }
}
