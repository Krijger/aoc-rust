use std::env;
use std::io::Result;
use std::process::exit;

use aoc::read_lines;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    match read_lines(file_path) {
        Ok(lines) => { 
            println!("Answer 1: {}", calculate_1(lines));
        }
        Err(e) => {
            eprintln!("Problem reading file {}: {}", file_path, e);
            exit(1);
        }
    }
}

fn calculate_1(lines: impl Iterator<Item = Result<String>>) -> u32 {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in lines {
        let text = line.unwrap();
        let mut parts = text.split_whitespace();
        left.push(parts.next().unwrap().parse().unwrap());
        right.push(parts.next().unwrap().parse().unwrap());
    }
    left.sort();
    right.sort();
    assert!(left.len() == right.len());

    let mut diff_sum = 0;
    while !left.is_empty() {
        diff_sum += i32::abs_diff(left.pop().unwrap(), right.pop().unwrap());
    }
    diff_sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::test_util::io_lines_from;

    #[test]
    fn test_example_1() -> Result<()> {
        let input = "
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
        ";
        assert_eq!(calculate_1(io_lines_from(input)), 11);
        Ok(())
    }
}