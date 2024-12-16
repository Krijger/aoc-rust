use std::env;
use std::process::exit;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Type {
    Space,
    Wall,
}

impl Type {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Self::Wall,
            '.' | 'S' | 'E' => Self::Space,
            _ => panic!("The map should not contain {}", c),
        }
    }
}

struct Maze { // TODO: use type ?
    map: Vec<Vec<Type>>,
}

impl Maze {
    fn new(lines: impl Iterator<Item = String>) -> Self {
        let mut map = Vec::new();
        for line in lines {
            map.push(line.chars().map(Type::from_char).collect());
        }
        Maze { map }
    }
    
    fn map_size(&self) -> (usize, usize) {
        let h = self.map.len();
        if h == 0 { panic!("Map of size 0 is unexpected")};
        let w = self.map.get(0).unwrap().len();
        if w == 0 { panic!("Map with first row of size 0 is unexpected")};
        (w, h)
    }

    #[allow(dead_code)]
    fn print(&self) {
        let size = self.map_size();
        for y in 0..size.1 {
            for x in 0..size.0 {
                print!("{}", match self.map.get(y).unwrap().get(x).unwrap() {
                    Type::Wall => "#",
                    Type::Space => ".",
                });
            }
            println!();       
        }
    }
}

fn calculate_a(lines: impl Iterator<Item = Result<String, std::io::Error>>) -> usize {
    let maze = Maze::new(lines.map(Result::unwrap));
    let start = (1, maze.map_size().1 - 2);
    let end = (maze.map_size().0 - 2, 1);

    maze.print();
    
    // every path is a set of points previously travelled (0), and a current position (1), current direction (2) and point total (3)
    // directions are 1,2,3,0, respective +x +y -x and -y or E S W N
    type Path = (Vec<(usize, usize)>, (usize, usize), usize, usize);
    let mut paths: Vec<Path> = Vec::new(); // TODO: use slices to avoid any copying, should be possible
    paths.push((Vec::new(), start, 1, 0));

    let ahead = |path: &Path| {
        match path.2 {
            1 => (path.1.0 + 1, path.1.1),
            2 => (path.1.0, path.1.1 + 1),
            3 => (path.1.0 - 1, path.1.1),
            0 => (path.1.0, path.1.1 - 1),
            x => panic!("illegal direction {}", x),
        }
    };
    let left = |path: &Path| {
        match path.2 {
            2 => (path.1.0 + 1, path.1.1),
            3 => (path.1.0, path.1.1 + 1),
            0 => (path.1.0 - 1, path.1.1),
            1 => (path.1.0, path.1.1 - 1),
            x => panic!("illegal direction {}", x),
        }
    };
    let right = |path: &Path| {
        match path.2 {
            0 => (path.1.0 + 1, path.1.1),
            1 => (path.1.0, path.1.1 + 1),
            2 => (path.1.0 - 1, path.1.1),
            3 => (path.1.0, path.1.1 - 1),
            x => panic!("illegal direction {}", x),
        }
    };
    let step_to = |path: &Path, to: (usize, usize), dir: usize, turned: bool| -> Option<Path> {
        // println!("Step to {:?},{:?}, in dir {}, already visited?: {}", to, maze.map.get(to.1)?.get(to.0)?, dir, path.0.iter().any(|&x| x == to));
        if !path.0.iter().any(|&x| x == to) && *maze.map.get(to.1)?.get(to.0)? == Type::Space {
            let mut travelled = path.0.clone();
            travelled.push(path.1);
            let score_increment = if turned { 1001 } else { 1 };
            Some((travelled, to, dir, path.3 + score_increment))
        } else {
            None
        }
    };

    let mut any_path_grown = true;
    // let mut i = 0;
    while any_path_grown {
        // i += 1;
        // println!("\n\n#############");
        // for path in &paths {
        //     println!("Path of length {}, current position {:?}, dir {}, and score {}:\n{:?}\n", path.0.len() + 1, path.1, path.2, path.3, path.0);
        // }
        any_path_grown = false;
        // always three possible moves: left or right (turn and step), or forward (step only)
        let mut new_paths: Vec<Path> = Vec::new();
        for path in &paths {
            if path.1 == end { // this path is already finished because at end location
                new_paths.push(path.clone());
            } else {
                if let Some(new) = step_to(&path, ahead(&path), path.2, false) {
                    new_paths.push(new);
                    any_path_grown = true;
                }
                if let Some(new) = step_to(&path, left(&path), (path.2 + 3) % 4, true) {
                    new_paths.push(new);
                    any_path_grown = true;
                }
                if let Some(new) = step_to(&path, right(&path), (path.2 + 1) % 4, true) {
                    new_paths.push(new);
                    any_path_grown = true;
                }
            }
        }
        if any_path_grown {
            paths = new_paths;
        }
    }
    
    paths.iter().filter(|p| p.1 == end).map(|p| p.3).min().unwrap()
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::test_util::io_lines_from;

    const INPUT: &str = "
            ###############
            #.......#....E#
            #.#.###.#.###.#
            #.....#.#...#.#
            #.###.#####.#.#
            #.#.#.......#.#
            #.#.#####.###.#
            #...........#.#
            ###.#.#####.#.#
            #...#.....#.#.#
            #.#.#.###.#.#.#
            #.....#...#.#.#
            #.###.#.#.#.#.#
            #S..#.....#...#
            ###############
        ";

    const LARGE_INPUT: &str = "
            #################
            #...#...#...#..E#
            #.#.#.#.#.#.#.#.#
            #.#.#.#...#...#.#
            #.#.#.#.###.#.#.#
            #...#.#.#.....#.#
            #.#.#.#.#.#####.#
            #.#...#.#.#.....#
            #.#.#####.#.###.#
            #.#.#.......#...#
            #.#.###.#####.###
            #.#.#...#.....#.#
            #.#.#.#####.###.#
            #.#.#.........#.#
            #.#.#.#########.#
            #S#.............#
            #################
    ";

    #[test]
    fn test_1() -> std::io::Result<()> {
        assert_eq!(calculate_a(io_lines_from(INPUT)), 7036);
        Ok(())
    }
    
    #[test]
    fn test_2() -> std::io::Result<()> {
        assert_eq!(calculate_a(io_lines_from(LARGE_INPUT)), 11048);
        Ok(())
    }
}
