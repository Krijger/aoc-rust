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

fn calculate(lines: impl Iterator<Item = Result<String, std::io::Error>>) -> usize {
    let map = read_map(lines.map(Result::unwrap).collect());
    map.print();
    let start = &map.find(|x| *x == 'S').unwrap();
    let end = &map.find(|x| *x == 'E').unwrap();
    let graph = graph_from_map(&map);

    println!("Graph has {} nodes", graph.nodes.len());

    let weight = |start: &Point, to: &Point| -> Option<usize> {
        if (start.0 == to.0 && start.1.abs_diff(to.1) == 1)
        || (start.1 == to.1 && start.0.abs_diff(to.0) == 1) {
            Some(1)
        } else {
            None
        }
    };

    graph.minimum_distance(
        |p| p.0 == start.0 && p.1 == start.1,
        |p| p.0 == end.0 && p.1 == end.1,
        weight)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    
    match aoc::read_lines(file_path) {
        Ok(lines) => { 
            let start = std::time::Instant::now();
            println!("Answer: {:?}", calculate(lines));
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
    fn test_regular_shortest_path() -> std::io::Result<()> {
        assert_eq!(calculate(io_lines_from(INPUT)), 84);
        Ok(())
    }
}
