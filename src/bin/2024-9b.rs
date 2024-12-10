use std::env;
use std::process::exit;

#[derive(Clone, Copy, Debug)]
struct Block {
    file: bool,
    size: u32,
    id: usize,
}

fn calculate(file_map: &str) -> usize {
    let mut blocks = decrypt(file_map);

    let next_id_to_process: usize = (blocks.len() + 1) / 2;
    for id in (0..next_id_to_process).rev() {
        let source = file_block_of_id(id, &blocks);
        if let Some(target) = open_block_of_size_index(source.1.size, &blocks) {
            if source.0 < target.0 { // we should only be moving stuff to the left
                continue;
            }

            let target_copy = (target.0, *target.1);
            let source_copy = (source.0, *source.1);
            
            // replace the source file block with open space before we start messing with the vector size
            blocks.splice(source.0 .. source.0 + 1, [Block {
                file: false,
                size: source.1.size,
                id: 0,
            }]);

            let replace_with = [source_copy.1, Block { // in case the target is the same size as the file block, this
                file: false,                      // will simply be a Block of size 0, which is fine
                size: target_copy.1.size - source_copy.1.size,
                id: 0,
            }];

            blocks.splice(target_copy.0 .. target_copy.0 + 1, replace_with);
        }
    }
    checksum(&blocks)
}

fn checksum(blocks: &Vec<Block>) -> usize {
    let mut x = 0;
    let mut index = 0;
    for b in blocks {
        for _ in 0..b.size {
            if b.file {
                x += index * b.id;
            }
            index += 1;
        }
    }
    x
}

fn decrypt(file_map: &str) -> Vec<Block> {
    file_map.chars().enumerate().map(|(i, c)| {
        Block {
            file: (i % 2) == 0,
            size: c.to_digit(10).unwrap(),
            id: i / 2,
        }
    }).collect()
}

fn file_block_of_id(id: usize, blocks: &[Block]) -> (usize, &Block) {
    for (i, block) in blocks.iter().enumerate() {
        if block.id == id && block.file {
            return (i, block)
        }
    }
    panic!("no block with id {} found", id);
}

fn open_block_of_size_index(min_size: u32, blocks: &[Block]) -> Option<(usize, &Block)> {
    blocks.iter().enumerate()
    .find(|(_, &block)| !block.file && block.size >= min_size)
}

#[allow(dead_code)]
fn print(blocks: &Vec<Block>) {
    for i in blocks {
        for _ in 0..i.size {
            match i.file {
                false => print!("."),
                true => print!("{}", i.id),
            }
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
            println!("Answer B: {}", solution);
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

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test() -> std::io::Result<()> {
        assert_eq!(calculate(&INPUT), 2858); // 6307279963620 for real data
        Ok(())
    }
}
