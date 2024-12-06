use std::collections::HashSet;
use std::env;
use std::process::exit;

use aoc::read_lines;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn turn(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
    fn look_ahead(&self, from: (usize, usize)) -> (usize, usize) {
        match self {
            Self::Up => (from.0, from.1 - 1),
            Self::Down => (from.0, from.1 + 1),
            Self::Left => (from.0 - 1, from.1),
            Self::Right => (from.0 + 1, from.1),
        }
    }
    fn looking_off_grid(&self, from: (usize, usize), max: (usize, usize)) -> bool {
        match self {
            Self::Up => from.1 == 0,
            Self::Down => from.1 == max.1,
            Self::Left => from.0 == 0,
            Self::Right => from.0 == max.0,
        }
    }
}

struct LabMap {
    obstacles: HashSet<(usize, usize)>,
    starting_pos: (usize, usize), 
    map_size: (usize, usize),
}

impl LabMap {
    pub fn new(lines: impl Iterator<Item = Result<String, std::io::Error>>) -> Self {
        let mut obstacles: HashSet<(usize, usize)> = HashSet::new();
        let mut starting_pos = (0, 0);
        let mut map_size = (0, 0);
    
        // parse input
        for (y, line) in lines.map(Result::unwrap).enumerate() {
            for (x, c) in line.chars().enumerate() {
                if y == 0 {
                    map_size.0 = x;
                }
                if c == '#' { 
                    obstacles.insert((x, y));
                }
                if c == '^' {
                    starting_pos = (x, y);
                }
            }
            map_size.1 = y;
        }
        LabMap {obstacles, starting_pos, map_size }
    }
}

fn calculate(lines: impl Iterator<Item = std::io::Result<String>>) -> (usize, usize) {
    let lab = LabMap::new(lines);

    let travelled_path = path(&lab, None).0;
    let answer_a = travelled_path.len();

    let loop_options = travelled_path.iter().filter(|&&object| {
        path(&lab, Some(object)).1
    }).count();

    (answer_a, loop_options)
}

// returns the travel path of the guard, and whether that path is a loop
// to find out whether a path is a loop you do not need to keep the path itself, but optimising that is not needed
fn path(lab: &LabMap, extra_obstacle: Option<(usize, usize)>) -> (HashSet<(usize, usize)>, bool) {
    let mut path: HashSet<(usize, usize)> = HashSet::new();
    let mut dir = Dir::Up;
    let mut guard = lab.starting_pos;
    let mut bumps: HashSet<((usize, usize), Dir)> = HashSet::new(); // bumped into (x, y), while going Dir
    loop {
        path.insert(guard);
        if dir.looking_off_grid(guard, lab.map_size) {
            break;
        }
        let mut ahead = dir.look_ahead(guard);
        while lab.obstacles.contains(&ahead) || ahead == extra_obstacle.unwrap_or((1000, 1000)) {
            if bumps.contains(&(ahead, dir)) { // already bumped like this, so must be in a loop
                return (path, true);
            };
            bumps.insert((ahead, dir));
            dir = dir.turn();
            ahead = dir.look_ahead(guard);
        }
        guard = ahead;
    }
    (path, false)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    match read_lines(file_path) {
        Ok(lines) => { 
            let solution = calculate(lines);
            println!("Answer A: {}, answer B: {}", solution.0, solution.1);
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
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
    ";

    #[test]
    fn test_a() -> std::io::Result<()> {
        assert_eq!(calculate(io_lines_from(INPUT)), (41, 6));
        Ok(())
    }
}
