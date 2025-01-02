use std::collections::{HashMap, HashSet};
use std::env;
use std::process::exit;

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
pub struct Graph<T> where 
    T: PartialEq,
{
    nodes: Vec<T>,
}

impl <T> Graph<T> where 
    T: PartialEq,
    T: Eq, // TODO: investigate if this is needed (for Dijkstra), with reference comparison as alternative
    T: std::hash::Hash,
{
    fn new() -> Self {
        Graph { nodes: Vec::new() }
    }
    
    fn connections<W>(&self, from: &T, weigth: &W) -> Vec<(&T, usize)> where 
        W: Fn(&T, &T) -> Option<usize>, // W gives weight of connection between two nodes, or None if not connected
    {
        self.nodes.iter().filter_map(|n| match weigth(from, n) {
            Some(w) => Some((n, w)),
            None => None,
        }).collect()
    }

    /// Dijkstra's algorithm to calculate minumum distance between `from` and `to`
    fn minimum_distance<W>(&self, from: &T, to: &T, weight: &W) -> usize where 
        W: Fn(&T, &T) -> Option<usize>, // W gives weight of connection between two nodes, or None if not connected
    {
        let mut distances: HashMap<&T, usize> = HashMap::new();
        let mut unvisiteds: HashSet<&T> = HashSet::new();
        for node in &self.nodes {
            distances.insert(node, if node == from { 0 } else {usize::MAX });
            unvisiteds.insert(node);
        }
        
        while !unvisiteds.is_empty() {
            let current_node = *unvisiteds.iter()
                .map(|unvisited| (unvisited, distances.get(unvisited).unwrap()))// TODO: this map can be removed
                .fold((None, usize::MAX),
                    |(closest, smallest_dist), (unvisited, dist)| {
                        if *dist < smallest_dist { (Some(unvisited), *dist) } else { (closest, smallest_dist) }
                    }
                )
                .0.unwrap();
            unvisiteds.remove(current_node);

            for (to, w) in self.connections(current_node, weight) {
                let curr_dist = distances.get(current_node).unwrap();
                if unvisiteds.contains(to) {
                    let to_distance = curr_dist + w;
                    if to_distance < *distances.get(to).unwrap() {
                        distances.insert(to, to_distance);
                    }
                }
            }
        }

        *distances.get(to).unwrap()
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

    fn weight(from: &Point, to: &Point) -> Option<usize> {
        if (from.0 == to.0 && from.1.abs_diff(to.1) == 1)
        || (from.1 == to.1 && from.0.abs_diff(to.0) == 1) {
            Some(1)
        } else {
            None
        }
    }

    graph.minimum_distance(start, end, &weight)
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
