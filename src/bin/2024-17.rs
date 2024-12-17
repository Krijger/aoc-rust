struct Register {
    a: u32,
    b: u32,
    c: u32,
}

fn calculate_a(reg: &mut Register, prog: &[u8]) -> String {
    let mut pointer = 0;
    let mut output = String::from("");
    while pointer < prog.len() {
        step(reg, prog, &mut pointer, &mut output);
    }

    output
}

fn step(reg: &mut Register, prog: &[u8], pointer: &mut usize, output: &mut String) {
    println!("Pointer {}, prog[*pointer] {}, prog[*pointer + 1] {}, combo(prog[*pointer + 1] {}",
        pointer, prog[*pointer], prog[*pointer + 1], combo(prog[*pointer + 1], &reg));
    match prog[*pointer] {
        0u8 => {
            reg.a /= 2_u32.pow(combo(prog[*pointer + 1], reg));
            *pointer += 2;
        }
        1u8 => {
            reg.b ^= prog[*pointer + 1] as u32;
            *pointer += 2;
        },
        2u8 => {
            reg.b = combo(prog[*pointer + 1], reg) % 8;
            *pointer += 2;
        },
        3u8 => {
            *pointer = if reg.a == 0 {
                *pointer + 2
            } else {
                prog[*pointer + 1] as usize
            }
        },
        4_u8 => {
            reg.b ^= reg.c;
            *pointer += 2;
        },
        5_u8 => {
            println!("combo(prog[*pointer + 1], reg) % 8: {}", combo(prog[*pointer + 1], reg) % 8);
            if output.len() > 0 { *output += ","; }
            *output += &(combo(prog[*pointer + 1], reg) % 8).to_string();
            *pointer += 2;
        },
        6_u8 => {
            reg.b = reg.a / 2_u32.pow(combo(prog[*pointer + 1], reg));
            *pointer += 2;
        },
        7_u8 => {
            reg.c = reg.a / 2_u32.pow(combo(prog[*pointer + 1], reg));
            *pointer += 2;
        },
        _ => { panic!("Invalid instruction: {}", prog[*pointer]) },
    }
}

fn combo(operand: u8, reg: &Register) -> u32 {
    match operand {
        0u8..=3u8 => operand as u32,
        4u8 => reg.a,
        5u8 => reg.b,
        6u8 => reg.c,
        _ => panic!("Invalid combo operand: {}", operand),
    }
}

fn main() {
    let start = std::time::Instant::now();

    let mut reg = Register { a: 44348299, b: 0, c: 0 };
    let prog = &[2,4,1,5,7,5,1,6,0,3,4,2,5,5,3,0];
    println!("Answer A: {}", calculate_a(&mut reg, prog));

    println!("Time elapsed in expensive_function() is: {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() -> std::io::Result<()> {
        let mut reg = Register { a: 0, b: 0, c: 9 };
        let prog = [2,6];
        let mut pointer = 0;
        let mut output = String::from("");
        step(&mut reg, &prog, &mut pointer, &mut output);
        assert_eq!(reg.b, 1);
        Ok(())
    }

    #[test]
    fn test_2() -> std::io::Result<()> {
        let mut reg = Register { a: 10, b: 0, c: 0 };
        let prog = &[5,0,5,1,5,4];
        assert_eq!(calculate_a(&mut reg, prog), "0,1,2");
        Ok(())
    }

    #[test]
    fn test() -> std::io::Result<()> {
        let mut reg = Register { a: 729, b: 0, c: 0 };
        let prog = &[0,1,5,4,3,0];
        assert_eq!(calculate_a(&mut reg, prog), "4,6,3,5,6,3,5,2,1,0");
        Ok(())
    }
}
