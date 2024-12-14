use std::env;
use std::process::exit;

#[derive(Debug)]
struct Robot((i32, i32), (i32, i32));

impl Robot {
    fn new(input: &str) -> Self {
        let (p, v) = input.split_once(" ").unwrap();
        let comma_p = p.find(",").unwrap();
        let comma_v = v.find(",").unwrap();

        Robot((
                p[2..comma_p].parse().unwrap(),
                p[comma_p+1..].parse().unwrap(),
            ),(
                v[2..comma_v].parse().unwrap(),
                v[comma_v+1..].parse().unwrap(),
            ))
    }
}

fn calculate_a(lines: impl Iterator<Item = Result<String, std::io::Error>>, map_size: (i32, i32)) -> usize {
    let mut robots: Vec<Robot> = lines.map(Result::unwrap).map(|s| Robot::new(&s)).collect();
    for s in 0..100 {
        robots = step(robots, map_size);
    }
    
    let x: i32 = (map_size.0 - 1) / 2;
    let y: i32 = (map_size.1 - 1) / 2;

    robots.iter().filter(|r| r.0.0 < x && r.0.1 < y).count() *
    robots.iter().filter(|r| r.0.0 > x && r.0.1 < y).count() *
    robots.iter().filter(|r| r.0.0 < x && r.0.1 > y).count() *
    robots.iter().filter(|r| r.0.0 > x && r.0.1 > y).count()
}

fn step(robots: Vec<Robot>, map_size: (i32, i32)) -> Vec<Robot> {
    robots.iter().map(|r| {
        Robot ((
            (r.0.0 + r.1.0).rem_euclid(map_size.0),
            (r.0.1 + r.1.1).rem_euclid(map_size.1),
        ), r.1)
    }).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    
    match aoc::read_lines(file_path) {
        Ok(lines) => { 
            let start = std::time::Instant::now();
            println!("Answer A: {}", calculate_a(lines, (101, 103)));
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
            p=0,4 v=3,-3
            p=6,3 v=-1,-3
            p=10,3 v=-1,2
            p=2,0 v=2,-1
            p=0,0 v=1,3
            p=3,0 v=-2,-2
            p=7,6 v=-1,-3
            p=3,0 v=-1,-2
            p=9,3 v=2,3
            p=7,3 v=-1,2
            p=2,4 v=2,-3
            p=9,5 v=-3,-3
        ";
        assert_eq!(calculate_a(io_lines_from(input), (11, 7)), 12);
        Ok(())
    }
}
