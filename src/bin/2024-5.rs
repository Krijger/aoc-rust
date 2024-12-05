use std::cmp::Ordering;
use std::collections::HashSet;
use std::env;
use std::process::exit;

use aoc::read_lines;

fn calculate_a(lines: impl Iterator<Item = std::io::Result<String>>) -> usize {
    let (instructions, updates) = parse_lines(lines);

    updates.iter()
    .filter(|u| {
        u.is_sorted_by(|x, y| comp_pages(x, y, &instructions))
    })
    .map(|u| u[(u.len() - 1) / 2])
    .sum()
}

#[allow(clippy::manual_inspect)] // https://github.com/rust-lang/rust-clippy/issues/13185
fn calculate_b(lines: impl Iterator<Item = std::io::Result<String>>) -> usize {
    let (instructions, mut updates) = parse_lines(lines);
    updates.iter_mut()
    .filter(|u| {
        !u.is_sorted_by(|x, y| comp_pages(x, y, &instructions))
    })
    .map(|u| {
        u.sort_by(|x, y| order(x, y, &instructions));
        u
    })
    .map(|u| u[(u.len() - 1) / 2])
    .sum()
}

fn order(p1: &usize, p2: &usize, instrs: &HashSet<(usize, usize)>) -> Ordering {
    // equal does not exist in this exercise
    if comp_pages(p1, p2, instrs) {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

fn comp_pages(p1: &usize, p2: &usize, instrs: &HashSet<(usize, usize)>) -> bool {
    instrs.contains(&(*p1, *p2))
}

fn parse_lines(lines: impl Iterator<Item = Result<String, std::io::Error>>) -> (HashSet<(usize, usize)>, Vec<Vec<usize>>) {
    let mut instructions = HashSet::new();
    let mut updates: Vec<Vec<usize>> = Vec::new();
    for line in lines.map(Result::unwrap) {
        if let Some(instr) = parse_instruction(&line) {
            instructions.insert(instr);
        }
        if let Some(update) = parse_update(&line) {
            updates.push(update);
        }
    }
    (instructions, updates)
}

fn parse_instruction(input: &str) -> Option<(usize, usize)> {
    input.split_once("|").map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
}

fn parse_update(input: &str) -> Option<Vec<usize>> {
    if input.contains(",") {
        Some(input.split(",").map(|n| n.parse::<usize>().unwrap()).collect())
    } else {
        None
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    match read_lines(file_path) {
        Ok(lines) => { 
            println!("Answer A: {}", calculate_a(lines));
        }
        Err(e) => {
            eprintln!("Problem reading file {}: {}", file_path, e);
            exit(1);
        }
    }

    match read_lines(file_path) {
        Ok(lines) => { 
            println!("Answer B: {}", calculate_b(lines));
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
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
    ";

    #[test]
    fn test_a() -> std::io::Result<()> {
        assert_eq!(calculate_a(io_lines_from(INPUT)), 143);
        Ok(())
    }

    #[test]
    fn test_b() -> std::io::Result<()> {
        assert_eq!(calculate_b(io_lines_from(INPUT)), 123);
        Ok(())
    }
}
