use std::env;
use std::process::exit;

use aoc::read_lines;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    match read_lines(file_path) {
        Ok(lines) => { 
            println!("Answer 1: {}", calculate_a(lines));
        }
        Err(e) => {
            eprintln!("Problem reading file {}: {}", file_path, e);
            exit(1);
        }
    }
}

fn calculate_a(lines: impl Iterator<Item = std::io::Result<String>>) -> usize {
    lines
        .map(Result::unwrap)
        .filter(|line| safe(line))
        .count()
}

fn safe(line: &str) -> bool {
    let init: (Option<bool>, Option<i32>, bool) = (None, None, true);
    line
        .split_whitespace()
        .map(|x| x.parse::<i32>())
        .map(Result::unwrap)
        .fold(init, |(increasing, last, stillgood), cur| {
            match (increasing, last, stillgood) {
                (_, None, _) => { (increasing, Some(cur), true) }
                (None, Some(l), _) => { (Some(cur > l), Some(cur), (cur - l).abs() > 0 && (cur - l).abs() <= 3) }
                (_, _, false) => { (increasing, last, false) }
                (Some(true), Some(l), true) => { (increasing, Some(cur), cur - l > 0 && cur - l <= 3) }
                (Some(false), Some(l), true) => { (increasing, Some(cur), l - cur > 0 && l - cur <= 3) }
            }
        })
        .2
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::test_util::io_lines_from;

    #[test]
    fn test_a() -> std::io::Result<()> {
        let input = "
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        ";
        assert_eq!(calculate_a(io_lines_from(input)), 2);
        Ok(())
    }
}
