use std::collections::HashSet;
use std::{env, usize};
use std::process::exit;

fn calculate_a(lines: impl Iterator<Item = Result<String, std::io::Error>>) -> usize {
    let lines: Vec<_> = lines.map(Result::unwrap).collect();
    let (towels, patterns) = parse_lines(&lines);

    patterns.into_iter().filter(|s| {
        pattern_possible_with_towels(&towels, s)
    })
    .count()
}

fn parse_lines<'a>(lines: &'a[String]) -> (Vec<&'a str>, Vec<&'a str>) {
    let towels = lines.get(0).unwrap().split(", ").collect();
    let patterns = lines[1..].iter().filter(|s| !s.is_empty()).map(String::as_str).collect();
    (towels, patterns)
}

fn pattern_possible_with_towels(towels: &[&str], pattern: &str) -> bool {
    let mut remainders: HashSet<&str> = HashSet::new();
    remainders.insert(pattern);

    while !remainders.iter().any(|remainder| remainder.is_empty()) {
        let mut new_options: HashSet<&str> = HashSet::new();
        for remainder in remainders {
            for &towel in towels {
                if remainder.starts_with(towel) {
                    // println!("Pattern/remainder {} starts with towel {}", remainder, towel);
                    new_options.insert(&remainder[towel.len()..]);
                }
            }
        }
        if new_options.is_empty() {
            return false;
        }
        remainders = new_options;
    }
    true
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    
    match aoc::read_lines(file_path) {
        Ok(lines) => { 
            let start = std::time::Instant::now();
            println!("Answer A: {}", calculate_a(lines));
            println!("Time elapsed in expensive_function() is: {:?}", start.elapsed());
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
            r, wr, b, g, bwu, rb, gb, br

            brwrr
            bggr
            gbbr
            rrbgbr
            ubwu
            bwurrg
            brgr
            bbrgwb
        ";

    #[test]
    fn test() -> std::io::Result<()> {
        assert_eq!(calculate_a(io_lines_from(INPUT)), 6);
        Ok(())
    }
}
