use std::{env, process::exit};
use aoc::read_string;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    match read_string(file_path) {
        Ok(input) => { 
            println!("Answer A: {}", calculate_a(input));
        }
        Err(e) => {
            eprintln!("Problem reading file {}: {}", file_path, e);
            exit(1);
        }
    }

    match read_string(file_path) {
        Ok(input) => { 
            println!("Answer B: {}", calculate_b(input));
        }
        Err(e) => {
            eprintln!("Problem reading file {}: {}", file_path, e);
            exit(1);
        }
    }
}

fn calculate_a(input: String) -> usize {
    input
        .split("mul(") // note we ignore a start with (x,y), because the data is not this edge case
        .map(value_for_str_after_mul_open)
        .sum()
}

fn calculate_b(input: String) -> usize {
    DoInstructionsSensitive(input)
        .collect::<Vec<_>>().iter()// TODO: clearly need to change the iteration Item to &str, but need to figure that out
        .flat_map(|do_part| do_part.split("mul(")) // again a start with (x,y) is ignored, because the data is not this edge case
        .map(value_for_str_after_mul_open)
        .sum()
}

fn value_for_str_after_mul_open(x: &str) -> usize {
    match x.split_once(",") {
        None => 0,
        Some((x1, x2)) => {
            x1.parse().unwrap_or(0) *
            match x2.split_once(")") {
                None => 0,
                Some((x21, _)) => {
                    x21.parse().unwrap_or(0)
                },
            }
        },
    }
}

struct DoInstructionsSensitive (String);

impl Iterator for DoInstructionsSensitive {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            return None;
        }
        let (next, new_self) = match self.0.split_once("don't()") {
            None => {
                (Some(self.0.clone()), String::from(""))
            },
            Some((pos, neg)) => {
                match neg.split_once("do()") {
                    None => { (Some(String::from(pos)), String::from("")) },
                    Some((_, poss)) => { (Some(String::from(pos)), String::from(poss)) },
                }
            },
        };
        self.0 = new_self;
        next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() -> std::io::Result<()> {
        let input = String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(calculate_a(input), 161);
        Ok(())
    }

    #[test]
    fn test_b() -> std::io::Result<()> {
        let input = String::from("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(calculate_b(input), 48);
        Ok(())
    }
}