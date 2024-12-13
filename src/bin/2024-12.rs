use std::collections::{HashMap, HashSet};
use std::env;
use std::process::exit;

use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point {
    x: usize,
    y: usize,
}

struct PlantMap(HashMap<Point, char>);

#[derive(PartialEq, Eq, Clone, Debug)]
struct Area {
    points: HashSet<Point>,
    t: char,
}

impl Area {
    fn new(p: Point, t: char) -> Self { 
        Self { points: vec![p].into_iter().collect(), t }
    }

    fn connected(&self, p: &Point, t: &char) -> bool {
        self.points.iter().any(|p2| {
            t == &self.t && (
                p.x == p2.x && (p.y + 1 == p2.y || p.y == p2.y + 1) ||
                p.y == p2.y && (p.x + 1 == p2.x || p.x == p2.x + 1)
            )
        })
    }

    // merge with point and other area(s), assume correct 'c'
    fn merge(&mut self, p: Point, others: Vec<Area>) {
        self.points.insert(p);
        others.into_iter().flat_map(|o| o.points).for_each(|p| {
            self.points.insert(p);
        });
    }

    fn score(&self) -> usize {
        let plant_count = self.points.len();
        let no_of_connections = self.points.iter()
            .permutations(2).unique()
            .map(|v| (v[0], v[1]))
            .filter(|(p1, p2)| {
                p1.x == p2.x && (p1.y + 1 == p2.y || p1.y == p2.y + 1) ||
                p1.y == p2.y && (p1.x + 1 == p2.x || p1.x == p2.x + 1)
            }).count();
        plant_count * (plant_count * 4 - no_of_connections)
    }

    fn score_b(&self) -> usize {
        let plant_count = self.points.len();

        #[derive(PartialEq, Eq, Hash, Debug)]
        struct Edge (Point, Point, u8); // the u8 is orientation: 1 2 3 4 not assigned to actual directions SWEN

        // calculate edges of length 1
        let mut edges: Vec<Edge> = self.points.iter()
            .flat_map(|p| {
                let mut edges: Vec<Edge> = Vec::new();
                if p.x == 0 || !self.points.contains(&Point{x: p.x - 1, y: p.y}) { edges.push(Edge(*p, Point{x: p.x, y: p.y + 1}, 1)) }
                if p.y == 0 || !self.points.contains(&Point{x: p.x, y: p.y - 1}) { edges.push(Edge(*p, Point{x: p.x + 1, y: p.y}, 2)) }

                if !self.points.contains(&Point{x: p.x + 1, y: p.y}) { edges.push(Edge(Point{x: p.x + 1, y: p.y}, Point{x: p.x + 1, y: p.y + 1}, 3)) }
                if !self.points.contains(&Point{x: p.x, y: p.y + 1}) { edges.push(Edge(Point{x: p.x, y: p.y + 1}, Point{x: p.x + 1, y: p.y + 1}, 4)) }

                edges
            })
            .collect();

        // now combine all connected aligned segment until no more such combinations exist
        let mut from = 1;
        'outer: while from < edges.len() {
            let e1 = &edges[from - 1];
            for (i, e2) in edges[from..].iter().enumerate() {
                if let Some(merged) = 
                        if e1.1 == e2.0 && e1.2 == e2.2 && (e1.0.x == e2.1.x || e1.0.y == e2.1.y) {
                            Some(Edge(e1.0, e2.1, e1.2))
                        } else if e1.0 == e2.1 && e1.2 == e2.2 && (e1.1.x == e2.0.x || e1.1.y == e2.0.y) {
                            Some(Edge(e1.1, e2.0, e1.2))
                        } else if e1.1 == e2.1 && e1.2 == e2.2 && (e1.0.x == e2.0.x || e1.0.y == e2.0.y) {
                            Some(Edge(e1.0, e2.0, e1.2))
                        } else if e1.0 == e2.0 && e1.2 == e2.2 && (e1.1.x == e2.1.x || e1.1.y == e2.1.y) {
                            Some(Edge(e1.1, e2.1, e1.2))
                        } else {
                            None
                        } {
                    edges[from - 1] = merged;
                    edges.remove(i + from);
                    continue 'outer;
                }
            }
            from += 1;
        }
        plant_count * edges.len()
    }
}

impl PlantMap {
    fn new(lines: impl Iterator<Item = Result<String, std::io::Error>>) -> Self {
        let mut plants: HashMap<Point, char> = HashMap::new();
        for (y, line) in lines.map(Result::unwrap).enumerate() {
            for (x, c) in line.chars().enumerate() {
                plants.insert(Point {x, y}, c);
            }
        }
        PlantMap(plants)
    }

    fn areas(&self) -> Vec<Area> {
        let mut areas: Vec<Area> = Vec::new();
        for (p, t) in &self.0 {
            let conn_areas: Vec<usize> = areas.iter()
                .enumerate()
                .filter(|&(_, a)| a.connected(p, t))
                .map(|(i, _)| i)
                .collect();
            match conn_areas.len() {
                0 => areas.push(Area::new(*p, *t)),
                n => {
                    let merge_with: Vec<Area> = conn_areas[1..n].iter().rev().map(|&i| {
                        let area = areas[i].clone();
                        areas.remove(i);
                        area
                    }).collect();
                    areas[conn_areas[0]].merge(*p, merge_with);
                },
            }
        }
        areas
    }
}

fn calculate(lines: impl Iterator<Item = Result<String, std::io::Error>>) -> usize {
    let map = PlantMap::new(lines);
    map.areas().iter().map(|a| a.score()).sum()
}

fn calculate_b(lines: impl Iterator<Item = Result<String, std::io::Error>>) -> usize {
    let map = PlantMap::new(lines);
    map.areas().iter().map(|a| a.score_b()).sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    
    match aoc::read_lines(file_path) {
        Ok(lines) => { 
            let start = std::time::Instant::now();
            println!("Answer A: {}", calculate(lines));
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
            println!("Answer B: {}", calculate_b(lines));
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

    #[test]
    fn test_1() -> std::io::Result<()> {
        let input = "
            AAAA
            BBCD
            BBCC
            EEEC
        ";
        assert_eq!(calculate(io_lines_from(input)), 140);
        assert_eq!(calculate_b(io_lines_from(input)), 80);
        Ok(())
    }

    #[test]
    fn test_2() -> std::io::Result<()> {
        let input = "
            OOOOO
            OXOXO
            OOOOO
            OXOXO
            OOOOO
        ";
        assert_eq!(calculate(io_lines_from(input)), 772);
        Ok(())
    }

    #[test]
    fn test_3() -> std::io::Result<()> {
        let input = "
            RRRRIICCFF
            RRRRIICCCF
            VVRRRCCFFF
            VVRCCCJFFF
            VVVVCJJCFE
            VVIVCCJJEE
            VVIIICJJEE
            MIIIIIJJEE
            MIIISIJEEE
            MMMISSJEEE
        ";
        assert_eq!(calculate(io_lines_from(input)), 1930);
        assert_eq!(calculate_b(io_lines_from(input)), 1206);
        Ok(())
    }

    #[test]
    fn test_4() -> std::io::Result<()> {
        let input = "
            EEEEE
            EXXXX
            EEEEE
            EXXXX
            EEEEE
        ";
        assert_eq!(calculate_b(io_lines_from(input)), 236);
        Ok(())
    }

    #[test]
    fn test_5() -> std::io::Result<()> {
        let input = "
            AAAAAA
            AAABBA
            AAABBA
            ABBAAA
            ABBAAA
            AAAAAA
        ";
        assert_eq!(calculate_b(io_lines_from(input)), 368);
        Ok(())
    }

}
