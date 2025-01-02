use std::env;
use std::process::exit;
use std::ptr::eq;

use aoc::map::{read_map, Mapp};

/// A non-directed graph, user must make sure not to insert edges (n1, n2) and (n2, n1). In
/// ```
/// pub struct Graph<T: PartialEq> {
///     nodes: Vec<T>,
///     edges: Vec<(T, T, usize)>,
/// }
/// ```
/// The `edges` contain copies of `T` already present in `nodes`, so there is a lot of Eq-ing, Hash-ing and such.
/// Profiling the code clearly showed this.
/// Changing the edges to contain references to the nodes is not possible due to
/// https://stackoverflow.com/questions/32300132/why-cant-i-store-a-value-and-a-reference-to-that-value-in-the-same-struct
/// https://stackoverflow.com/questions/28608823/how-to-model-complex-recursive-data-structures-graphs
/// so the alternative is to make a struct owning the nodes and methods to the needed graphing
/// specifically optimised for out algorithm.
pub struct Graph<T>
{
    nodes: Vec<T>,
}

impl <T> Graph<T>
{
    fn new() -> Self {
        Graph { nodes: Vec::new() }
    }
    
    fn connections<W>(&self, from: &T, weigth: &W) -> Vec<(&T, usize)> where 
        W: Fn(&T, &T) -> Option<usize>, // W gives weight of connection between two nodes, or None if not connected
    {
        self.nodes.iter()
        .filter_map(|n| weigth(from, n).map(|w| (n, w)))
        .collect()
    }

    /// Dijkstra's algorithm to calculate minumum distance between `start` and `end`.
    /// Uses https://doc.rust-lang.org/std/ptr/fn.eq.html internally to compare &Ts, which is why the start and end
    /// are supplied as functions (because raw pointers should point to graph.nodes entries, which could easily be
    /// mistaken) - in case is_start / is_end does not return true for any graph node, this function will panic
    fn minimum_distance<S, E, W>(&self, is_start: S, is_end: E, weight: W) -> usize where
        S: Fn(&T) -> bool,
        E: Fn(&T) -> bool,
        W: Fn(&T, &T) -> Option<usize>, // W gives weight of connection between two nodes, or None if not connected
    {
        // `table` contains references to all nodes (table.0), and as prescribed by the Dijkstra's algorithm:
        // a distance (table.1), and the visited status (table.2)
        let mut table: Vec<(&T, usize, bool)> = self.nodes.iter()
            .map(|n| (n, usize::MAX, false))
            .collect();

        let start_index = table.iter().position(|(n, _, _)| is_start(n)).unwrap();
        table[start_index].1 = 0;
        
        while table.iter().any(|(_, _, visited)| !visited) {
            let current_node = table.iter()
                .filter(|(_, _, visited)| !visited )
                .fold((None, usize::MAX),
                    |(min_dist_node, min_dist), (n, d, _)| {
                        if *d < min_dist { (Some(*n), *d) } 
                        else { (min_dist_node, min_dist) }
                })
                .0.unwrap();

            let current_node_index = table.iter().position(|(n, _, _)| eq(*n, current_node)).unwrap();
            table[current_node_index].2 = true;

            // for all connections to current node, check if the route via the current node is the first or
            // in case a route already exists, whether the current route is shorter. In both cases, update
            // the distance of the connected node in our table
            let curr_dist = table[current_node_index].1;
            for (to, w) in self.connections(current_node, &weight) {
                let to_node_index = table.iter().position(|(n, _, _)| eq(*n, to)).unwrap();
                if !table[to_node_index].2 {
                    let to_distance_via_curr_node = curr_dist + w;
                    if to_distance_via_curr_node < table[to_node_index].1 {
                        table[to_node_index].1 = to_distance_via_curr_node;
                    }
                }
            }
        }

        let end_index = table.iter().position(|(n, _, _)| is_end(n)).unwrap();
        table[end_index].1
    }
}

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
