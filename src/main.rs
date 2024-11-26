use std::env;
use std::fs::File;
use std::io::BufReader;
use std::process::exit;

use aoc::read_lines;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    match read_lines(file_path) {
        Ok(lines) => { 
            calculate(lines);
        }
        Err(e) => {
            eprintln!("Problem reading file {}: {}", file_path, e);
            exit(1);
        }
    }
}

fn calculate(lines: std::io::Lines<BufReader<File>>) {
    let _ = lines
            .map(Result::unwrap)
            .for_each(|x| println!("{x}"));
}
