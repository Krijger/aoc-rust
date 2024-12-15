use std::collections::HashMap;
use std::env;
use std::process::exit;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point(usize, usize);

#[derive(PartialEq, Debug, Clone, Copy)]
enum Type {
    Space,
    Wall,
    Boks,
    Robot,
}

struct Warehouse(HashMap<Point, Type>, Point, Vec<char>);

impl Type {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Self::Wall,
            '.' => Self::Space,
            'O' => Self::Boks,
            '@' => Self::Robot,
            _ => panic!("The map should not contain {}", c),
        }
    }
}

impl Warehouse {
    fn new(lines: impl Iterator<Item = String>) -> Self {
        let mut w: HashMap<Point, Type> = HashMap::new();
        let mut robot = Point(0, 0);
        let mut instructions = Vec::new();
        lines.enumerate().for_each(|(y, line)| {
            if line.starts_with(">") || line.starts_with("<") || line.starts_with("v") || line.starts_with("^") {
                for c in line.chars() {
                    instructions.push(c);
                }
            } else {
                line.chars().map(Type::from_char).enumerate().for_each(|(x, t)| {
                    match t {
                        Type::Robot => {
                            robot = Point(x, y);
                            w.insert(Point(x, y), Type::Space);
                        },
                        _ => { w.insert(Point(x, y), t); },
                    }
                });
            }
        });
        instructions.reverse();
        Warehouse(w, robot, instructions)
    }

    fn direction(&self, c: char, offset: usize) -> Option<(Point, Type)> {
        let n = match c {
            '>' => Point(self.1.0 + offset, self.1.1),
            '<' => Point(self.1.0 - offset, self.1.1),
            '^' => Point(self.1.0, self.1.1 - offset),
            'v' => Point(self.1.0, self.1.1 + offset),
            _ => panic!("unexpected instruction"),
        };
        match self.0.get(&n) {
            None => return None,
            Some(&t) => return Some((n, t)),
        }
    }

    fn next_space_before_wall(&self, c: char) -> Option<Point> {
        let mut offset = 1;
        loop {
            // println!("{}, {}, {:?}", c, offset, self.direction(c, offset));
            match self.direction(c, offset) {
                None => return None,
                Some((p, Type::Space)) => return Some(p),
                Some((_, Type::Wall)) => return None,
                _ => {}, // continue loop to check next point
            }
            offset += 1;
        }
    }

    fn step(&mut self) {
        let instr = self.2.pop().unwrap();
        match self.direction(instr, 1) {
            None | Some((_, Type::Wall)) => {},
            Some((p, Type::Space)) => { self.1 = p; },
            Some((p, Type::Boks)) => { 
                // println!("{} : self.next_space_before_wall(instr): {:?}", instr, self.next_space_before_wall(instr));
                if let Some(next_space) = self.next_space_before_wall(instr) {
                    self.0.insert(next_space, Type::Boks);
                    self.0.insert(p, Type::Space);
                    self.1 = p; 
                }
            },
            Some((_, Type::Robot)) => panic!("should not run into a robot"),
        }
        // println!("Postion Robot: {:?} after instruction {}", self.1, instr);
    }

    fn score(&self) -> usize {
        self.0.iter()
        .filter(|&(_, &t)| t == Type::Boks)
        .map(|(p, _)| p.1 * 100 + p.0)
        .sum()
    }
}

fn calculate_a(lines: impl Iterator<Item = Result<String, std::io::Error>>) -> usize {
    let mut w = Warehouse::new(lines.map(Result::unwrap));
    // println!("{:?}", w.0.values());
    while w.2.len() > 0 {
        w.step();
    }
    w.score()
}

fn calculate_b(lines: impl Iterator<Item = Result<String, std::io::Error>>) -> usize {
    let mut w = Warehouse::new(lines.map(Result::unwrap));
    // println!("{:?}", w.0.values());
    while w.2.len() > 0 {
        w.step();
    }
    w.score()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    
    match aoc::read_lines(file_path) {
        Ok(lines) => { 
            let start = std::time::Instant::now();
            println!("Answer A: {}", calculate_a(lines));
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
            println!("Answer A: {}", calculate_b(lines));
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
            ########
            #..O.O.#
            ##@.O..#
            #...O..#
            #.#.O..#
            #...O..#
            #......#
            ########

            <^^>>>vv<v>>v<<
        ";

    #[test]
    fn test_1() -> std::io::Result<()> {
        
        assert_eq!(calculate_a(io_lines_from(INPUT)), 2028);
        Ok(())
    }
}
