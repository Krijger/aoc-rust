use std::env;
use std::process::exit;

fn calculate(stone_str: &str, max_rank: usize) -> u64 {

    // let mut v: Vec<(usize, &str)> = stone_str.split_ascii_whitespace().map(|s| {
    //     (0, s)
    // }).collect();

    // let mut n = 0;
    // while let Some((rank, stone)) = v.pop() {
    //     if rank == 25 {
    //         n += 1;
    //     } else {
    //         match stone {
    //             "0" => {
    //                 v.push((rank + 1, "1"));
    //             }
    //             _ if stone.len() % 2 == 0 => {
    //                 let len = stone.len() / 2;
    //                 v.push((rank + 1, &stone[..len]));
    //                 let mut right = &stone[(len + 1)..];
    //                 while right.len() > 1 && right.starts_with("0") {
    //                     right = &right[1..];
    //                 }
    //                 v.push((rank + 1, right));
    //             }
    //             _ => {
    //                 let new_num: u32 = stone.parse::<u32>().unwrap() * 2024;
    //                 let new = new_num.to_string();
    //                 v.push((rank + 1, &new));
    //             }
    //         }
    //     }
    // }

    let mut v: Vec<(usize, String)> = stone_str.split_ascii_whitespace().map(|s| {
        (0, String::from(s))
    }).collect();

    let mut n = 0;
    while let Some((rank, stone)) = v.pop() {
        if rank == max_rank {
            // println!("{}", stone);
            n += 1;
        } else {
            if stone == String::from("0") {
                v.push((rank + 1, String::from("1")));
            } else if &stone.len() % 2 == 0 {
                let len = stone.len() / 2;
                v.push((rank + 1, String::from(&stone[..len])));
                let mut right = &stone[len..];
                while right.len() > 1 && right.starts_with("0") { // remove leading 0's
                    right = &right[1..];
                }
                v.push((rank + 1, String::from(right)));
            } else {
                let new_num: u64 = stone.parse::<u64>().unwrap() * 2024;
                let new = new_num.to_string();
                v.push((rank + 1, new));
            }
        }
    }
    n
}

fn main() {
    let start = std::time::Instant::now();
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    match aoc::read_string(file_path) {
        Ok(line) => { 
            println!("Answer A: {}", calculate(&line, 25));
            println!("Answer B: {}", calculate(&line, 75));
            // for i in 0..26 {
            //     println!("Answer A: {}: {}", i, calculate("2 54 992917 5270417 2514 28561 0 990", i));
            // }
            
        }
        Err(e) => {
            eprintln!("Problem reading file {}: {}", file_path, e);
            exit(1);
        }
    }

    println!("Time elapsed in expensive_function() is: {:?}", start.elapsed());
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
