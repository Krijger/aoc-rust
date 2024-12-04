use std::collections::HashMap;
use std::env;
use std::process::exit;

use aoc::read_lines;

enum Xmas { X, M, A, S }
impl Xmas {
    fn from_char(s: char) -> Result<Self, String> {
        match s {
            'X' => Ok(Xmas::X),
            'M' => Ok(Xmas::M),
            'A' => Ok(Xmas::A),
            'S' => Ok(Xmas::S),
            _ => Err(format!("Could not parse {} to XMAS", s)),
        }
    }
}

fn build_map(lines: impl Iterator<Item = Result<String, std::io::Error>>) -> HashMap<(isize, isize), Xmas> {
    let mut map: HashMap<(isize, isize), Xmas> = HashMap::new();
    lines.enumerate().for_each(|(y, line)| {
        line.unwrap().chars().enumerate().for_each(|(x, c)| {
            map.insert((x.try_into().unwrap(), y.try_into().unwrap()), Xmas::from_char(c).unwrap());
        });
    });
    map
}

fn calculate_a(lines: impl Iterator<Item = std::io::Result<String>>) -> usize {
    let map = build_map(lines);
    map
        .keys()
        .flat_map(move |xy| {
            [(-1, -1), (0, -1), (1, -1),
            (-1, 0)          , (1, 0) ,
            (-1, 1) , (0, 1) , (1, 1) , ].iter().map(move |dir| (xy, dir))
        })
        .filter( |(xy, dir)| {
            matches!((
                map.get(xy), 
                map.get(&(xy.0 + dir.0, xy.1 + dir.1)),
                map.get(&(xy.0 + dir.0 * 2, xy.1 + dir.1 * 2)),
                map.get(&(xy.0 + dir.0 * 3, xy.1 + dir.1 * 3))
            ), (Some(Xmas::X), Some(Xmas::M), Some(Xmas::A), Some(Xmas::S)))
        })
        .count()
}

fn calculate_b(lines: impl Iterator<Item = std::io::Result<String>>) -> usize {
    let map = build_map(lines);
    map.keys().map(|xy| {
        match map.get(xy) {
            Some(Xmas::A) => {},
            _ => { return 0; },
        };
        match (map.get(&(xy.0 - 1, xy.1 - 1)), map.get(&(xy.0 + 1, xy.1 + 1))) {
            (Some(Xmas::M), Some(Xmas::S)) | (Some(Xmas::S), Some(Xmas::M)) => {
                match (map.get(&(xy.0 - 1, xy.1 + 1)), map.get(&(xy.0 + 1, xy.1 - 1))) {
                    (Some(Xmas::M), Some(Xmas::S)) | (Some(Xmas::S), Some(Xmas::M)) => 1,
                    _ => 0,
                }
            },
            _ => 0,
        }
    }).sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    match read_lines(file_path) {
        Ok(lines) => { 
            println!("Answer A: {}", calculate_a(lines));
        }
        Err(e) => {
            eprintln!("Problem reading file {}: {}", file_path, e);
            exit(1);
        }
    }

    match read_lines(file_path) {
        Ok(lines) => { 
            println!("Answer B: {}", calculate_b(lines));
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
    fn test_a_oxymoron() -> std::io::Result<()> {
        let input = "
            SAMXMAS
        ";
        assert_eq!(calculate_a(io_lines_from(input)), 2);
        Ok(())
    }

    #[test]
    fn test_a() -> std::io::Result<()> {
        let input = "
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX
        ";
        assert_eq!(calculate_a(io_lines_from(input)), 18);
        Ok(())
    }

    #[test]
    fn test_b() -> std::io::Result<()> {
        let input = "
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX
        ";
        assert_eq!(calculate_b(io_lines_from(input)), 9);
        Ok(())
    }
}
