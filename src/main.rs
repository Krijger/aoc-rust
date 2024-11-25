use std::env;

use aoc::read_lines;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("Using file {file_path}");

    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            println!("{}", line);
        }
    }
}
