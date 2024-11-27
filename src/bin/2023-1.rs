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

fn calculate_1(lines: std::io::Lines<BufReader<File>>) -> i32 {
    lines
        .map(Result::unwrap)
        .map(|line| value_for_line_1(&line))
        .sum()
}

fn calculate_2(lines: std::io::Lines<BufReader<File>>) -> i32 {
    lines
        .map(Result::unwrap)
        .map(|line| value_for_line_2(&line))
        .sum()
}

fn value_for_line_1(line: &str) -> i32 {
    let (mut start, mut end) = (line, line);

    let left = loop {
        if start.starts_with("1") {
            break "1";
        } else if start.starts_with("2") {
            break "2";
        } else if start.starts_with("3") {
            break "3";
        } else if start.starts_with("4") {
            break "4";
        } else if start.starts_with("5") {
            break "5";
        } else if start.starts_with("6") {
            break "6";
        } else if start.starts_with("7") {
            break "7";
        } else if start.starts_with("8") {
            break "8";
        } else if start.starts_with("9") {
            break "9";
        }
    
        start = &start[1..start.len()]; // this only works because the input is UTF-8
    };

    let right = loop {
        if end.ends_with("1") {
            break "1";
        } else if end.ends_with("2") {
            break "2";
        } else if end.ends_with("3") {
            break "3";
        } else if end.ends_with("4") {
            break "4";
        } else if end.ends_with("5") {
            break "5";
        } else if end.ends_with("6") {
            break "6";
        } else if end.ends_with("7") {
            break "7";
        } else if end.ends_with("8") {
            break "8";
        } else if end.ends_with("9") {
            break "9";
        }
    
        end = &end[0..end.len() - 1]; // this only works because the input is UTF-8
    };

    format!("{}{}", left, right).parse().unwrap()
}

fn value_for_line_2(line: &str) -> i32 {
    let (mut start, mut end) = (line, line);

    let left = loop {
        if start.starts_with("1") || start.starts_with("one") {
            break "1";
        } else if start.starts_with("2") || start.starts_with("two") {
            break "2";
        } else if start.starts_with("3") || start.starts_with("three") {
            break "3";
        } else if start.starts_with("4") || start.starts_with("four") {
            break "4";
        } else if start.starts_with("5") || start.starts_with("five") {
            break "5";
        } else if start.starts_with("6") || start.starts_with("six") {
            break "6";
        } else if start.starts_with("7") || start.starts_with("seven") {
            break "7";
        } else if start.starts_with("8") || start.starts_with("eight") {
            break "8";
        } else if start.starts_with("9") || start.starts_with("nine") {
            break "9";
        }
    
        start = &start[1..start.len()]; // this only works because the input is ASCII
    };

    let right = loop {
        if end.ends_with("1") || end.ends_with("one") {
            break "1";
        } else if end.ends_with("2") || end.ends_with("two") {
            break "2";
        } else if end.ends_with("3") || end.ends_with("three") {
            break "3";
        } else if end.ends_with("4") || end.ends_with("four") {
            break "4";
        } else if end.ends_with("5") || end.ends_with("five") {
            break "5";
        } else if end.ends_with("6") || end.ends_with("six") {
            break "6";
        } else if end.ends_with("7") || end.ends_with("seven") {
            break "7";
        } else if end.ends_with("8") || end.ends_with("eight") {
            break "8";
        } else if end.ends_with("9") || end.ends_with("nine") {
            break "9";
        }
    
        end = &end[0..end.len() - 1]; // this only works because the input is ASCII
    };

    format!("{}{}", left, right).parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_example_1() -> std::io::Result<()> {
        assert_eq!(calculate_1(read_lines(&"tst-input/2023-1-test-1.txt".to_string())?), 142);
        Ok(())
    }

    #[test]
    fn test_example_2() -> std::io::Result<()> {
        assert_eq!(calculate_2(read_lines(&"tst-input/2023-1-test-2.txt".to_string())?), 281);
        Ok(())
    }
}
