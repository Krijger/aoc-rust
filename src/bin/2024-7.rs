use std::collections::HashSet;
use std::env;
use std::process::exit;

use aoc::read_lines;

struct Cal {
    to_process: Vec<usize>,
}

impl Cal {
    pub fn new(input: &str) -> Self {
        Cal { 
            to_process: input.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect(),
        }
    }
    pub fn process(&mut self, allow_concat: bool) -> HashSet<usize> {
        let mut processed = HashSet::new();
        for &n in &self.to_process {
            if processed.is_empty() {
                processed.insert(n);
            } else {
                let mut inner = HashSet::new();
                for x in processed {
                    inner.insert(x + n);
                    inner.insert(x * n);
                    if allow_concat {
                        inner.insert((x.to_string() + &n.to_string()).parse().unwrap());
                    }
                }
                processed = inner;
            }
        }
        processed
    }
}

fn calculate(lines: impl Iterator<Item = std::io::Result<String>>) -> (usize, usize) {
    lines.map(Result::unwrap).map(|line| {
        if let Some((output, inputs)) = line.split_once(":") {
            let mut cal = Cal::new(inputs);
            let output = output.parse().unwrap();
            return match (cal.process(false).contains(&output), cal.process(true).contains(&output)) {
                (true, true) => (output, output),
                (false, true) => (0, output),
                _ => (0, 0),
            }
        } 
        (0, 0)
    }).fold((0, 0), |x, y| (x.0 + y.0, x.1 + y.1))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    match read_lines(file_path) {
        Ok(lines) => { 
            let solution = calculate(lines);
            println!("Answer A: {}, answer B: {}", solution.0, solution.1);
        }
        Err(e) => {
            eprintln!("Problem reading file {}: {}", file_path, e);
            exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::test_util::io_lines_from;

    const INPUT: &str = "
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
    ";

    #[test]
    fn test() -> std::io::Result<()> {
        assert_eq!(calculate(io_lines_from(INPUT)), (3749, 11387));
        Ok(())
    }
}
