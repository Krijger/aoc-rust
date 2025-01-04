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

    let graph = graph_from_map(&map);
    
    let mut cheats: Vec<(&Point, &Point)> = Vec::new();
    for n1 in &graph.nodes {
        for n2 in &graph.nodes {
            let cheat_dist = n1.0.abs_diff(n2.0) + n1.1.abs_diff(n2.1);
            if cheat_dist == 2
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

    // TODO: allow `graph.minimum_distance` for a subgraph, with a filter for the nodes.
    // That gives a way to upper limit in our exercise:
    //      there always is a connection between 2 points (race track property)
    //      if no connection is found in a subgraph of radius T/2 around the center of the points
    //      then the min_dist between those points must be > T
    //      So for T = 100, and points 2 steps apart, take a subgraph of radius 51 and find a connection
    // Also, allow for a lower limit (carthesian distance in our example), that short cicuits
    // the distance calculation, because you know even smaller distance is impossible
    // Also, can Dijkstra be made more efficient in case the end node is known?

    cheats.iter()
    .enumerate()
    .inspect(|(i, _)| println!("Processing cheat {} of {}", i, cheats.len()))
    .map(|(_, cheat)| {
        graph.minimum_distance(
            |p| p.0 == cheat.0.0 && p.1 == cheat.0.1,
            |p| p.0 == cheat.1.0 && p.1 == cheat.1.1,
            |from, to| base_weight(from, to)
        )
    })
    .inspect(|cheat_dist| println!("Cheat distance: {}", cheat_dist))
    .map(|cheat_dist| cheat_dist - 2)
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
        assert_eq!(calculate(io_lines_from(INPUT), 2), 44);
        assert_eq!(calculate(io_lines_from(INPUT), 12), 8);
        assert_eq!(calculate(io_lines_from(INPUT), 64), 1);
        Ok(())
    }
}
