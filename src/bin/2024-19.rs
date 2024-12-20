use std::collections::HashMap;
use std::{env, usize};
use std::process::exit;

struct RemaindersWithCounts<'a>(HashMap<&'a str, usize>); // TODO: generalize to CountedSet<Item>
impl <'a>RemaindersWithCounts<'a> {
    fn new() -> Self {
        Self(HashMap::new())
    }
    fn new_with_pattern(pattern: &'a str) -> Self {
        let mut value = Self::new();
        value.insert((pattern, 1));
        value
    }
    fn insert(&mut self, (remainder, count): (&'a str, usize)) {
        match self.0.get(&remainder) {
            None => self.0.insert(remainder, count),
            Some(old_count) => self.0.insert(remainder, old_count + count),
        };
    }
}

fn calculate(lines: impl Iterator<Item = Result<String, std::io::Error>>) -> (usize, usize) {
    let lines: Vec<_> = lines.map(Result::unwrap).collect();
    let (towels, patterns) = parse_lines(&lines);

    patterns.into_iter().map(|s| {
        match number_of_solutions(&towels, s) {
            0 => (0, 0),
            n => (1, n),
        }
    }).fold((0, 0), |agg, other| (agg.0 + other.0, agg.1 + other.1))
}

fn parse_lines<'a>(lines: &'a[String]) -> (Vec<&'a str>, Vec<&'a str>) {
    let towels = lines.get(0).unwrap().split(", ").collect();
    let patterns = lines[1..].iter().filter(|s| !s.is_empty()).map(String::as_str).collect();
    (towels, patterns)
}

fn number_of_solutions(towels: &[&str], pattern: &str) -> usize {
    let mut remainders_with_count = RemaindersWithCounts::new_with_pattern(pattern);
    while remainders_with_count.0.keys().any(|rem| !rem.is_empty()) {
        let mut tmp = RemaindersWithCounts::new();
        for remainder in remainders_with_count.0 {
            // keep count of the valid answers, because all other remainders without a match are thrown away
            if remainder.0.is_empty() {                 
                tmp.insert(remainder);
            }
            for &towel in towels {
                if remainder.0.starts_with(towel) {
                    tmp.insert((&remainder.0[towel.len()..], remainder.1));
                }
            }
        }
        if tmp.0.is_empty() {
            return 0;
        } else {
            remainders_with_count = tmp;
        }
    }
    *remainders_with_count.0.get("").unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    
    match aoc::read_lines(file_path) {
        Ok(lines) => { 
            let start = std::time::Instant::now();
            println!("Answer: {:?}", calculate(lines));
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
        assert_eq!(calculate(io_lines_from(INPUT)), (6, 16));
        Ok(())
    }
}
