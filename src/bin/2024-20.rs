use std::collections::{HashMap, HashSet};
use std::env;
use std::process::exit;

use aoc::map::{read_map, Mapp};

/// A non-directed graph, user must make sure not to insert edges (n1, n2) and (n2, n1)
pub struct Graph<T: PartialEq> {
    nodes: Vec<T>,
    edges: Vec<(T, T, usize)>,
}

type Point = (usize, usize);

impl <T: PartialEq> Graph<T> {
    fn new() -> Self {
        Graph { nodes: Vec::new(), edges: Vec::new()}
    }
    fn connections(&self, from: &T) -> Vec<(&T, &usize)> {
        let mut conns = Vec::new();
        for edge in &self.edges {
            match edge {
                (n, to, w) if n == from => conns.push((to, w)),
                (to, n, w) if n == from => conns.push((to, w)),
                _ => {},
            }
        }
        conns
    }
}

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
    for y in 0..height {
        for x in 0..width {
            if !graph.nodes.contains(&(x, y)) {
                continue;
            }
            if x > 0 && graph.nodes.contains(&(x - 1, y)) {
                graph.edges.push(((x - 1, y), (x,y), 1));
            }
            if y > 0 && graph.nodes.contains(&(x, y - 1)) {
                graph.edges.push(((x, y - 1), (x,y), 1));
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

    println!("Graph has {} nodes and {} edges", graph.nodes.len(), graph.edges.len());

    let mut distances: HashMap<&Point, usize> = HashMap::new();
    let mut unvisited: HashSet<&Point> = HashSet::new();
    for node in &graph.nodes {
        distances.insert(node, if node == start { 0 } else {usize::MAX });
        unvisited.insert(node);
    }
    
    while !unvisited.is_empty() {
        let current = unvisited_node_with_min_dist(&distances, &unvisited);
        for (to, w) in graph.connections(current) {
            let curr_dist = distances.get(current).unwrap();
            if unvisited.contains(to) {
                let to_distance = curr_dist + w;
                if to_distance < *distances.get(to).unwrap() {
                    distances.insert(to, to_distance);
                }
            }
            unvisited.remove(current);
        }
    }

    *distances.get(end).unwrap()
}

fn unvisited_node_with_min_dist<'a>(distances: &HashMap<&'a Point, usize>, unvisited: &HashSet<&'a Point>) -> &'a Point {
    unvisited.iter()
        .map(|unvisited_point| (unvisited_point, distances.get(unvisited_point).unwrap()))
        .fold((&(usize::MAX, usize::MAX), usize::MAX), |acc, x| if *x.1 < acc.1 { (*x.0, *x.1) } else { acc })
        .0
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
