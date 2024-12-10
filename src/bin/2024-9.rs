use std::env;
use std::process::exit;

fn calculate(file_map: &str) -> (usize, usize) {
    let mut blocks = decrypt(file_map);

    loop {
        match first_space_index(&blocks) {
            None => break,
            Some(x) => {
                if x == blocks.len() - 1 { // this open space is the last element
                    break;
                }
                blocks[x] = blocks.pop().unwrap();
            },
        }
    }

    let answer_a = blocks.iter().enumerate().filter(|(_, &id)| id.is_some()).map(|(i, id)| i * id.unwrap()).sum();

    (answer_a, 0)
}

fn decrypt(file_map: &str) -> Vec<Option<usize>> {
    let mut blocks = Vec::new();
    let mut empty_spots = Vec::new();
    let mut id = 0;
    for (i, c) in file_map.chars().enumerate() {
        let is_file_block = (i % 2) == 0;
        let digit = c.to_digit(10).unwrap();
        for _ in 0..digit {
            if is_file_block {
                blocks.push(Some(id));
            } else {
                blocks.push(None);
                empty_spots.push(blocks.len() - 1);
            }
        }
        if is_file_block {
            id += 1;
        }
    }
    blocks
}

fn first_space_index(blocks: &[Option<usize>]) -> Option<usize> {
    for (i, &block) in blocks.iter().enumerate() {
        if block.is_none() {
            return Some(i);
        }
    }
    None
}

#[allow(dead_code)]
fn print(blocks: &Vec<Option<usize>>) {
    for i in blocks {
        match i {
            None => print!("."),
            Some(x) => print!("{}", x),
        }
    }
    println!();
}

fn main() {
    let start = std::time::Instant::now();
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    match aoc::read_string(file_path) {
        Ok(line) => { 

            let solution = calculate(&line);
            println!("Answer A: {}, answer B: {}", solution.0, solution.1);
        }
        Err(e) => {
            eprintln!("Problem reading file {}: {}", file_path, e);
            exit(1);
        }
    }
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test() -> std::io::Result<()> {
        assert_eq!(calculate(&INPUT), (1928, 0));
        Ok(())
    }
}
