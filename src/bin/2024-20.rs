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
            let are_vertically_sandwiching_a_wall = n1.0 == n2.0
                    && n1.1.abs_diff(n2.1) == 2 
                    && map.get(n1.0, (n1.1 + n2.1) / 2).is_some_and(|c| *c == '#');
            let are_horizontally_sandwiching_a_wall = n1.1 == n2.1
                    && n1.0.abs_diff(n2.0) == 2 
                    && map.get((n1.0 + n2.0) / 2, n1.1).is_some_and(|c| *c == '#');
            // let cheat_dist = n1.0.abs_diff(n2.0) + n1.1.abs_diff(n2.1);
            if (are_vertically_sandwiching_a_wall || are_horizontally_sandwiching_a_wall)
            // if cheat_dist == 2
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

    // Done: allow `graph.minimum_distance` for a subgraph, with a filter for the nodes.
    //      That gives a way to upper limit in our exercise:
    //      there always is a connection between 2 points (race track property)
    //      if no connection is found in a subgraph of radius T/2 around the center of the points
    //      then the min_dist between those points must be > T
    //      So for T = 100, and points 2 steps apart, take a subgraph of radius 51 (+1 for rounding errors) 
    //      and find a connection
    // Done: allow for a lower limit (carthesian distance in our example), that short cicuits
    //      the distance calculation, because you know even smaller distance is impossible
    // Also, can Dijkstra be made more efficient in case the end node is known?

    cheats.iter()
    .enumerate()
    .inspect(|(i, _)| println!("Processing cheat {} of {}", i, cheats.len()))
    .map(|(_, cheat)| {
        let center = ((cheat.0.0 + cheat.1.0) / 2, (cheat.0.1 + cheat.1.1) / 2);
        let cart_dist = cheat.0.0.abs_diff(cheat.1.0) + cheat.0.1.abs_diff(cheat.1.1);
        let subgraph_nodes = graph.nodes.iter()
            .filter(|&n| n.0.abs_diff(center.0) + n.1.abs_diff(center.1) <= min_time_saved + 1)
            .collect();
        let subgraph = Graph { nodes: subgraph_nodes };
        let dist = subgraph.minimum_distance_bounded(
            |p| p.0 == cheat.0.0 && p.1 == cheat.0.1,
            |p| p.0 == cheat.1.0 && p.1 == cheat.1.1,
            |from, to| base_weight(from, to),
            min_time_saved + cart_dist
        ).unwrap_or(usize::MAX); // if not connected, we know the distance saved is larger than our minimum
        dist - cart_dist
    })
    .inspect(|time_saved| println!("Cheat saves time: {}", time_saved))
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
