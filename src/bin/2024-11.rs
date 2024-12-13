use std::collections::HashMap;
use std::env;
use std::process::exit;

fn calculate(input: &str, max_rank: usize) -> u64 {

    let mut stones: HashMap<u64, u64> = HashMap::new();
    input.split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .for_each(|x| {
            stones.insert(x, stones.get(&x).unwrap_or(&0) + 1);
        });

    for _ in 0..max_rank {
        let mut new_stones: HashMap<u64, u64> = HashMap::new();
        for (stone, count) in stones.iter() {
            let stone_str = stone.to_string();
            let stones_after: Vec<u64> = 
                if *stone == 0 {
                    vec![1]
                } else if stone_str.len() % 2 == 0 {
                    let len = stone_str.len() / 2;
                    vec![stone_str[..len].parse::<u64>().unwrap(), stone_str[len..].parse::<u64>().unwrap()]
                } else {
                    vec![stone * 2024]
                };
            for new_stone in stones_after {
                new_stones.insert(new_stone, new_stones.get(&new_stone).unwrap_or(&0) + count);
            }
        }
        stones = new_stones;
    }
    stones.values().sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    
    match aoc::read_string(file_path) {
        Ok(line) => { 
            let start = std::time::Instant::now();
            println!("Answer A: {}", calculate(&line, 25));
            println!("Answer B: {}", calculate(&line, 75));
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

    #[test]
    fn test() -> std::io::Result<()> {
        assert_eq!(calculate("125 17", 25), 55312);
        Ok(())
    }

    #[test]
    fn test_leading_zeroes() -> std::io::Result<()> {
        assert_eq!(calculate("1000", 2), 3); // would be 4 if 1000 was split into 10 and 00
        Ok(())
    }
}
