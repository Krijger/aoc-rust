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

fn calculate(lines: impl Iterator<Item = Result<String, std::io::Error>>, min_time_saved: usize) -> usize {
    let map = read_map(lines.map(Result::unwrap).collect());
    let start = &map.find(|x| *x == 'S').unwrap();
    let end = &map.find(|x| *x == 'E').unwrap();

    let graph = graph_from_map(&map);
    
    // TODO: possible to make this much more efficient for larger `min_time_saved` by analysing the local map
    // a bit further.
    // Now that I think of it, a more general approach is to find to shortest distance of the cheat itself, as
    // it will save time of that shortest distance minus 2. This is true only because the regular race track
    // is a single path
    let mut cheats: Vec<(&Point, &Point)> = Vec::new();
    for n1 in &graph.nodes {
        for n2 in &graph.nodes {
            let are_vertically_sandwiching_a_wall = n1.0 == n2.0
                    && n1.1.abs_diff(n2.1) == 2 
                    && map.get(n1.0, (n1.1 + n2.1) / 2).is_some_and(|c| *c == '#');
            let are_horizontally_sandwiching_a_wall = n1.1 == n2.1
                    && n1.0.abs_diff(n2.0) == 2 
                    && map.get((n1.0 + n2.0) / 2, n1.1).is_some_and(|c| *c == '#');
            if (are_vertically_sandwiching_a_wall || are_horizontally_sandwiching_a_wall)
            && !cheats.contains(&(n2, n1)) {
                cheats.push((n1, n2));
            }
        }
    }

    let base_weight = |from: &Point, to: &Point| -> Option<usize> {
        if (from.0 == to.0 && from.1.abs_diff(to.1) == 1)
        || (from.1 == to.1 && from.0.abs_diff(to.0) == 1) {
            Some(1)
        } else {
            None
        }
    };

    let benchmark = graph.minimum_distance(
        |p| p.0 == start.0 && p.1 == start.1,
        |p| p.0 == end.0 && p.1 == end.1,
        base_weight);

    cheats.iter()
    .enumerate()
    .inspect(|(i, _)| println!("Processing cheat {} of {}", i, cheats.len()))
    .map(|(_, cheat)| {
        graph.minimum_distance(
            |p| p.0 == start.0 && p.1 == start.1,
            |p| p.0 == end.0 && p.1 == end.1,
            |from, to| {
                if from == cheat.0 && to == cheat.1
                || from == cheat.1 && to == cheat.0 {
                    Some(2)
                } else {
                    base_weight(from, to)
                }
            })
    })
    .map(|min_dist| benchmark - min_dist)
    .filter(|time_saved| *time_saved >= min_time_saved)
    .count()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    
    match aoc::read_lines(file_path) {
        Ok(lines) => { 
            let start = std::time::Instant::now();
            println!("Answer: {:?}", calculate(lines, 100));
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
        assert_eq!(calculate(io_lines_from(INPUT), 0), 44);
        assert_eq!(calculate(io_lines_from(INPUT), 12), 8);
        assert_eq!(calculate(io_lines_from(INPUT), 64), 1);
        Ok(())
    }
}
