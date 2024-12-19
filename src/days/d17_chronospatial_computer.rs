use itertools::Itertools;

use crate::{
    day,
    utils::{read_lines, Day, Task},
};

fn parse_input(filename: &str) -> ([u32; 3], Vec<u32>) {
    fn parse_register(line: String) -> u32 {
        let (_, v) = line.split_once(':').unwrap();
        v.trim().parse().unwrap()
    }

    let mut lines = read_lines(filename);
    let a = parse_register(lines.next().unwrap());
    let b = parse_register(lines.next().unwrap());
    let c = parse_register(lines.next().unwrap());

    lines.next();

    let program = lines.next().unwrap();
    let (_, program) = program.split_once(':').unwrap();
    let program = program.trim().split(',').flat_map(str::parse::<u32>).collect();

    ([a, b, c], program)
}

fn combo(op: u32, regs: &[u32; 3]) -> u32 {
    match op {
        0..4 => op,
        4..7 => regs[op as usize - 4],
        _ => unreachable!(),
    }
}

fn p1(filename: &str) -> String {
    let (mut regs, program) = parse_input(filename);
    let mut pointer = 0;

    std::iter::from_fn(|| {
        let instruction = *program.get(pointer)?;
        let op = *program.get(pointer + 1)?;

        match instruction {
            // adv
            0 => regs[0] /= 2u32.pow(combo(op, &regs)),
            // bxl
            1 => regs[1] ^= op,
            // bst
            2 => regs[1] = combo(op, &regs) % 8,
            // jnz
            3 if regs[0] == 0 => {}
            3 => {
                pointer = op as usize;
                return Some(None);
            }
            // bxc
            4 => regs[1] ^= regs[2],
            // out
            5 => {
                pointer += 2;
                return Some(Some(combo(op, &regs) % 8));
            }
            // bdv
            6 => regs[1] = regs[0] / 2u32.pow(combo(op, &regs)),
            // cdv
            7 => regs[2] = regs[0] / 2u32.pow(combo(op, &regs)),
            _ => unreachable!(),
        }

        pointer += 2;
        Some(None)
    })
    .flatten()
    .join(",")
}

fn p2(filename: &str) -> u32 {
    // let (mut regs, program) = parse_input(filename);
    // let mut pointer = 0;

    0
}

pub const SOLUTION: Day<String, u32> = day! { 17,
    part_1: { examples: ["example_1.txt"], func: p1 },
    part_2: { examples: ["example_2.txt"], func: p2 }
};

#[cfg(test)]
mod d17_tests {
    use super::*;

    #[test]
    fn p1_example_tests() {
        assert_eq!(&SOLUTION.part_1.run_example(0), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn p2_example_tests() {
        assert_eq!(SOLUTION.part_2.run_example(0), 117440);
    }

    #[test]
    fn playground() {
        let res = p1(SOLUTION.part_1.examples[0]);
        println!("{res}");
    }
}
