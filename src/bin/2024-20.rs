use std::env;
use std::process::exit;

use aoc::graph::Graph;
use aoc::map::{read_map, Mapp};

type Point = (usize, usize);

fn graph_from_map(map: &Mapp<char>) -> Graph<Point> {
    let mut graph = Graph::new();
    let height = map.height();
    let width = map.width();
    for y in 0..height {
        for x in 0..width {
            match map.get(x, y) {
                Some('S') | Some('E') | Some('.') => graph.nodes.push((x, y)),
                _ => {},
            }
        }
    }
    graph
}

fn calculate(lines: impl Iterator<Item = Result<String, std::io::Error>>, min_time_saved: usize, max_cheat_length: usize) -> usize {
    let map = read_map(lines.map(Result::unwrap).collect());
    let start = map.find(|c| *c == 'S').unwrap();
    let is_start = |node: &Point| { 
        node.0 == start.0 && node.1 == start.1 
    };
    let graph = graph_from_map(&map);
    let weight = |from: &Point, to: &Point| {
        if (from.0 == to.0 && from.1.abs_diff(to.1) == 1)
        || (from.1 == to.1 && from.0.abs_diff(to.0) == 1) {
            Some(1)
        } else {
            None
        }
    };
    let distances = graph.distances(is_start, weight);
    let distance = |n: &Point| {
        distances.iter().find(|(other, _)| other.0 == n.0 && other.1 == n.1).map(|(_, d)| d)
    };
    
    let mut valid_cheat_count = 0;
    for n1 in &graph.nodes {
        for n2 in &graph.nodes {
            let carthesian_dist = n1.0.abs_diff(n2.0) + n1.1.abs_diff(n2.1);
            let is_cheat = carthesian_dist <= max_cheat_length;
            if is_cheat {
                match (distance(n1), distance(n2)) {
                    (Some(d1), Some(d2)) if d2 > d1 => {
                        let time_saved = d2 - d1 - carthesian_dist;
                        if time_saved >= min_time_saved {
                            valid_cheat_count += 1;
                        }
                    },
                    _ => {}, 
                }
            }
        }
    }

    valid_cheat_count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    
    match aoc::read_lines(file_path) {
        Ok(lines) => { 
            let start = std::time::Instant::now();
            println!("Answer: {:?}", calculate(lines, 100, 2));
            println!("Time elapsed in expensive_function() is: {:?}", start.elapsed());
        }
        Err(e) => {
            eprintln!("Problem reading file {}: {}", file_path, e);
            exit(1);
        }
    }
    
    match aoc::read_lines(file_path) {
        Ok(lines) => { 
            let start = std::time::Instant::now();
            println!("Answer: {:?}", calculate(lines, 100, 20));
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
    use aoc::test_util::io_lines_from;

    const INPUT: &str = "
            ###############
            #...#...#.....#
            #.#.#.#.#.###.#
            #S#...#.#.#...#
            #######.#.#.###
            #######.#.#...#
            #######.#.###.#
            ###..E#...#...#
            ###.#######.###
            #...###...#...#
            #.#####.#.###.#
            #.#...#.#.#...#
            #.#.#.#.#.#.###
            #...#...#...###
            ###############
        ";

    #[test]
    fn test_a() -> std::io::Result<()> {
        assert_eq!(calculate(io_lines_from(INPUT), 2, 2), 44);
        assert_eq!(calculate(io_lines_from(INPUT), 12, 2), 8);
        assert_eq!(calculate(io_lines_from(INPUT), 64, 2), 1);
        Ok(())
    }

    #[test]
    fn test_b() -> std::io::Result<()> {
        assert_eq!(calculate(io_lines_from(INPUT), 50, 20), 285);
        assert_eq!(calculate(io_lines_from(INPUT), 70, 20), 41);
        assert_eq!(calculate(io_lines_from(INPUT), 76,20 ), 3);
        Ok(())
    }
}
