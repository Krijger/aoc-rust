use std::{env, process::exit};
use aoc::read_string;


fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    match read_string(file_path) {
        Ok(input) => { 
            println!("Answer 1: {}", calculate_a(input));
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
        .map(|x| {
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
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() -> std::io::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(calculate_a(String::from(input)), 161);
        Ok(())
    }
}