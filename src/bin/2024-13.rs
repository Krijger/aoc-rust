use std::cmp::min;
use std::env;
use std::process::exit;

struct Machine {
    a: (u64, u64),
    b: (u64, u64),
    p: (u64, u64),
}

impl Machine {
    fn min_tokens(&self) -> u64 {
        for a in 0..min(self.p.0.div_ceil(self.a.0), self.p.1.div_ceil(self.a.1)) {
            for b in 0..min(self.p.0.div_ceil(self.b.0), self.p.1.div_ceil(self.b.1)) {
                if a * self.a.0 + b * self.b.0 == self.p.0 &&
                   a * self.a.1 + b * self.b.1 == self.p.1 {
                    return a * 3 + b;
                }
            }   
        }
        0
    }

    /*
    // x steps of A and y steps of B must be P:
    x a0 + y b0 = p1        (1)
    x a1 + y b1 = p2        (2)

    // transform (2)
    x = (p2 - y b1) / a1    (3)

    // to substitute x into (1)., proceed to find y from known numbers in (4)
    (p2 - y b1) a0 / a1 + y b0 = p1
    a0 p2 / a1 - y b1 a0 / a1 + y b0 = p1
    y ( b0 - b1 a0 / a1 ) = p1 - a0 p2 / a1
    y = (p1 - p2 a0 / a1) / (b0 - b1 a0 / a1)   (4)

    // and remember (3) to find x
    x = (p2 - y b1) / a1
    */
    fn min_tokens_efficient(&self) -> u64 {
        let (p1, p2) = (self.p.0 as f64, self.p.1 as f64);
        let (a0, a1) = (self.a.0 as f64, self.a.1 as f64);
        let (b0, b1) = (self.b.0 as f64, self.b.1 as f64);
        let y = (p1 - p2 * a0 / a1) / (b0 - b1 * a0 / a1);
        let x = (p2 - y * b1) / a1;
        if ((x - (x as u64) as f64) < 0.001 || ((x - (x as u64) as f64) > 0.999)) 
        && ((y - (y as u64) as f64) < 0.001 || ((y - (y as u64) as f64) > 0.999)) {
            3 * x.round() as u64 + y.round() as u64
        } else {
            0
        }
    }
}

fn calculate_a(lines: impl Iterator<Item = Result<String, std::io::Error>>) -> u64 {
    parse_machines(lines).iter().map(Machine::min_tokens).sum()
}

#[allow(clippy::manual_inspect)] // https://github.com/rust-lang/rust-clippy/issues/13185
fn calculate_b(lines: impl Iterator<Item = Result<String, std::io::Error>>) -> u64 {
    parse_machines(lines).iter_mut()
    .map(|m| {
        m.p.0 += 10000000000000;
        m.p.1 += 10000000000000;
        m
    })
    .map(|m| m.min_tokens_efficient())
    .sum()
}

fn parse_machines(lines: impl Iterator<Item = Result<String, std::io::Error>>) -> Vec<Machine> {
    let mut machines: Vec<Machine> = Vec::new();
    let mut a_buts: Vec<(u64, u64)> = Vec::new();
    let mut b_buts: Vec<(u64, u64)> = Vec::new();
    let mut prizes: Vec<(u64, u64)> = Vec::new();
    // TODO: this can be done way nicer by interweaving 4 iterators I guess. Find out how.
    lines.map(Result::unwrap).filter(|line| !line.is_empty()).enumerate().for_each(|(i, line)| {
        match i % 3 {
            0 => { // A
                let (left, y) = line.split_once(", Y+").unwrap();
                let (_, x) = left.split_once("X+").unwrap();
                let parsed = (x.parse().unwrap(), y.parse().unwrap());
                a_buts.push(parsed);
            },
            1 => { // B
                let (left, y) = line.split_once(", Y+").unwrap();
                let (_, x) = left.split_once("X+").unwrap();
                let parsed = (x.parse().unwrap(), y.parse().unwrap());
                b_buts.push(parsed);
            },
            2 => { // P
                let (left, y) = line.split_once(", Y=").unwrap();
                let (_, x) = left.split_once("X=").unwrap();
                let parsed = (x.parse().unwrap(), y.parse().unwrap());
                prizes.push(parsed);
            },
            _ => { }, // empty line
        }
    });
    for i in 0..a_buts.len() {
        machines.push(Machine { a: a_buts[i], b: b_buts[i], p: prizes[i] });
    }
    machines
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
            Button A: X+94, Y+34
            Button B: X+22, Y+67
            Prize: X=8400, Y=5400

            Button A: X+26, Y+66
            Button B: X+67, Y+21
            Prize: X=12748, Y=12176

            Button A: X+17, Y+86
            Button B: X+84, Y+37
            Prize: X=7870, Y=6450

            Button A: X+69, Y+23
            Button B: X+27, Y+71
            Prize: X=18641, Y=10279
        ";
        assert_eq!(calculate_a(io_lines_from(input)), 480);
        Ok(())
    }
}
