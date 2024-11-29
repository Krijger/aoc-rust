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

    match read_lines(file_path) {
        Ok(lines) => { 
            println!("Answer 2: {}", calculate_2(lines));
        }
        Err(e) => {
            eprintln!("Problem reading file {}: {}", file_path, e);
            exit(1);
        }
    }
}

fn calculate_1(lines: impl Iterator<Item = Result<String>>) -> i32 {
    lines
        .map(Result::unwrap)
        .map(|line| value_for_line_1(&line))
        .sum()
}

fn calculate_2(lines: impl Iterator<Item = Result<String>>) -> i32 {
    lines
        .map(Result::unwrap)
        .map(|line| value_for_line_2(&line))
        .sum()
}

fn value_for_line_1(line: &str) -> i32 {

    let digits = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];

    let len = line.len();
    let (mut left, mut right): (Option<&str>, Option<&str>) = (None, None);

    'outer: for i in 0..len {
        let tail = &line[i..len]; // this only works because the input is ASCII
        for &digit in &digits {
            if tail.starts_with(digit) {
                left = Some(digit);
                break 'outer;
            }
        };
    }

    'outer: for i in 0..len {
        let head = &line[0..len-i]; // this only works because the input is ASCII
        for &digit in &digits {
            if head.ends_with(digit) {
                right = Some(digit);
                break 'outer;
            }
        };
    }

    format!("{}{}", left.unwrap_or("0"), right.unwrap_or("0")).parse().unwrap()
}

fn value_for_line_2(line: &str) -> i32 {

    let digits = vec![
        ("1", "one"),
        ("2", "two"),
        ("3", "three"),
        ("4", "four"),
        ("5", "five"),
        ("6", "six"),
        ("7", "seven"),
        ("8", "eight"),
        ("9", "nine"),
    ];

    let len = line.len();
    let (mut left, mut right): (Option<&str>, Option<&str>) = (None, None);

    'outer: for i in 0..len {
        let tail = &line[i..len]; // this only works because the input is ASCII
        for &digit in &digits {
            if tail.starts_with(digit.0) || tail.starts_with(digit.1) {
                left = Some(digit.0);
                break 'outer;
            }
        };
    }

    'outer: for i in 0..len {
        let head = &line[0..len-i]; // this only works because the input is ASCII
        for &digit in &digits {
            if head.ends_with(digit.0) || head.ends_with(digit.1) {
                right = Some(digit.0);
                break 'outer;
            }
        };
    }

    format!("{}{}", left.unwrap_or("0"), right.unwrap_or("0")).parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn io_lines_from<'a>(input: &'a str) -> impl Iterator<Item = Result<String>> + 'a {
        input.lines()
            .map(|line| line.trim())
            .filter(|&line| !line.is_empty())
            .map(|line| Result::Ok(String::from(line)))
    }

    #[test]
    fn test_example_1() -> Result<()> {
        let input = "
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        ";
        assert_eq!(calculate_1(io_lines_from(input)), 142);
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<()> {
        let input = "
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        ";
        assert_eq!(calculate_2(io_lines_from(input)), 281);
        Ok(())
    }
}
