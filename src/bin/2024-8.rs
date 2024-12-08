use std::collections::{HashMap, HashSet};
use std::env;
use std::process::exit;

use itertools::Itertools;

use aoc::read_lines;

struct Roof {
    antennas: HashMap<char, HashSet<Coord>>,
    map_size: (usize, usize),
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

impl Roof {
    pub fn new(lines: impl Iterator<Item = Result<String, std::io::Error>>) -> Self {
        let mut antennas: HashMap<char, HashSet<Coord>> = HashMap::new();
        let mut map_size = (0, 0);
        
        // parse input
        for (y, line) in lines.map(Result::unwrap).enumerate() {
            map_size.1 = y;
            for (x, c) in line.chars().enumerate() {
                if y == 0 {
                    map_size.0 = x;
                }
                if c == '.' {
                    continue;
                }
                if !antennas.contains_key(&c) {
                    antennas.insert(c, HashSet::new());
                }
                antennas.get_mut(&c).unwrap().insert(Coord{x, y});
            }
        }

        Roof { antennas, map_size }
    }
}

fn calculate(lines: impl Iterator<Item = std::io::Result<String>>) -> (usize, usize) {
    let roof = Roof::new(lines);
    let mut antinodes_a: HashSet<Coord> = HashSet::new();
    let mut antinodes_b: HashSet<Coord> = HashSet::new();
    for (_, ants_of_freq) in roof.antennas {
        for mut pair in ants_of_freq.iter().permutations(2) { // (a,b) and (b,a) are both generated
            let a = pair.pop().unwrap();
            let b = pair.pop().unwrap();
            let diff_x = b.x as i32 - a.x as i32;
            let diff_y = b.y as i32 - a.y as i32;

            fn next(cur: &Coord, diff_x: &i32, diff_y: &i32, map_size: &(usize, usize)) -> Option<Coord> {
                let x = cur.x as i32 - diff_x;
                let y = cur.y as i32 - diff_y;
                if x >= 0 && y >= 0 && x <= map_size.0 as i32 && y <= map_size.1 as i32 {
                    Some(Coord { x: x as usize, y: y as usize })
                } else {
                    None
                }
            }
            
            let mut z = next(a, &diff_x, &diff_y, &roof.map_size);
            if let Some(coord) = z { 
                antinodes_a.insert(coord); 
            }

            antinodes_b.insert(*a);
            while let Some(coord) = z {
                antinodes_b.insert(coord);
                z = next(&coord, &diff_x, &diff_y, &roof.map_size);
            }

        }
    }
    (antinodes_a.len(), antinodes_b.len())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    match read_lines(file_path) {
        Ok(lines) => { 
            let solution = calculate(lines);
            println!("Answer A: {}, answer B: {}", solution.0, solution.1);
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
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
    ";

    #[test]
    fn test() -> std::io::Result<()> {
        assert_eq!(calculate(io_lines_from(INPUT)), (14, 34));
        Ok(())
    }
}
