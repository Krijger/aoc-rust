use std::collections::HashMap;

struct TrailMap {
    heights: HashMap<Coord, u32>,
    map_size: (usize, usize),
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

impl TrailMap {
    pub fn new(lines: impl Iterator<Item = Result<String, std::io::Error>>) -> Self {
        let mut heights: HashMap<Coord, u32> = HashMap::new();
        let mut map_size = (0, 0);
        
        // parse input
        for (y, line) in lines.map(Result::unwrap).enumerate() {
            map_size.1 = y;
            for (x, c) in line.chars().enumerate() {
                if y == 0 {
                    map_size.0 = x;
                }
                let h = c.to_digit(10).unwrap();
                heights.insert(Coord{x, y}, h);
            }
        }

        TrailMap { heights, map_size }
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for y in 0..self.map_size.1 {
            for x in 0..self.map_size.0 {
                print!("{}", self.heights.get(&Coord{x, y}).unwrap());
            }
            println!();
        }
    }

    pub fn one_higher(&self, cur_h: u32, cur_level: Vec<&Coord>) -> Vec<&Coord> {
        self.heights.iter()
        .filter(|&(cand, cand_h)| {
            *cand_h == cur_h + 1 && 
            cur_level.iter()
            .any(|cur| {
                cand.x == cur.x && (cand.y + 1 == cur.y || cand.y == cur.y + 1) ||
                cand.y == cur.y && (cand.x + 1 == cur.x || cand.x == cur.x + 1)
            })
        })
        .map(|(cand, _)| cand)
        .collect()
    }
}

fn calculate(lines: impl Iterator<Item = Result<String, std::io::Error>>) -> usize {
    let tm = TrailMap::new(lines);
    let mut current_level: Vec<_> = tm.heights.iter().filter(|&(_, &h)| h == 0).map(|(c, _)| c).collect(); // lvl 0
    for height in 0..9 {
        current_level = current_level.iter().flat_map(|c| {
            tm.one_higher(height, vec![c])
        })
        .collect();
    }
    current_level.len()
}

fn main() {
    let start = std::time::Instant::now();
    let args: Vec<String> = std::env::args().collect();

    let file_path = &args[1];

    match aoc::read_lines(file_path) {
        Ok(lines) => { 

            let solution = calculate(lines);
            println!("Answer B: {}", solution);
        }
        Err(e) => {
            eprintln!("Problem reading file {}: {}", file_path, e);
            std::process::exit(1);
        }
    }
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::test_util::io_lines_from;

    const INPUT: &str = "
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732
    ";

    #[test]
    fn test() -> std::io::Result<()> {
        assert_eq!(calculate(io_lines_from(INPUT)), 81);
        Ok(())
    }
}


