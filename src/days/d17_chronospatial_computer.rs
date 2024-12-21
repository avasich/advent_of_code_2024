use itertools::Itertools;

use crate::{
    day,
    utils::{Day, Task, read_lines},
};

fn parse_input(filename: &str) -> ([u64; 3], Vec<u64>) {
    fn parse_register(line: String) -> u64 {
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
    let program = program.trim().split(',').flat_map(str::parse::<_>).collect();

    ([a, b, c], program)
}

fn combo(op: u64, regs: &[u64; 3]) -> u64 {
    match op {
        0..4 => op,
        4..7 => regs[op as usize - 4],
        _ => unreachable!(),
    }
}

fn execute(regs: &mut [u64; 3], program: &[u64]) -> impl Iterator<Item = u64> {
    let mut pointer = 0;

    std::iter::from_fn(move || {
        let instruction = *program.get(pointer)?;
        let op = *program.get(pointer + 1)?;
        pointer += 2;

        match instruction {
            0 => regs[0] /= 2u64.pow(combo(op, regs) as u32), // adv
            1 => regs[1] ^= op,                               // bxl
            2 => regs[1] = combo(op, regs) % 8,               // bst
            3 if regs[0] == 0 => {}                           // jnz
            3 => {
                pointer = op as usize;
                return Some(None);
            }
            4 => regs[1] ^= regs[2],                                   // bxc
            5 => return Some(Some((combo(op, regs) % 8, *regs))),      // out
            6 => regs[1] = regs[0] / 2u64.pow(combo(op, regs) as u32), // bdv
            7 => regs[2] = regs[0] / 2u64.pow(combo(op, regs) as u32), // cdv
            _ => unreachable!(),
        }
        Some(None)
    })
    .flatten()
    .map(|(v, _)| v)
}

fn p1(filename: &str) -> String {
    let (mut regs, program) = parse_input(filename);

    execute(&mut regs, &program).join(",")
}

fn p2(filename: &str) -> u64 {
    let (_, program) = parse_input(filename);
    let start_reg = 8_u64.pow(program.len() as u32) - 1;
    traverse(start_reg, &program, 0).unwrap()
}

fn traverse(reg: u64, program: &[u64], d: usize) -> Option<u64> {
    if d == program.len() {
        return Some(reg);
    }
    let d0 = program.len() - d - 1;
    (0..8).find_map(|x| {
        let p = 8_u64.pow(d0 as u32);
        let reg = reg - (reg / p % 8) * p + x * p;
        let c = std::iter::zip(execute(&mut [reg, 0, 0], program), program.iter())
            .skip(d0)
            .filter(|&(a, &b)| a == b)
            .count();
        (c == d + 1).then(|| traverse(reg, program, d + 1)).flatten()
    })
}

pub const SOLUTION: Day<String, u64> = day! { 17,
    part_1: { examples: ["example_1.txt", "example_2.txt"], func: p1 },
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
        // task:   88645131227352 too low
        //       4774437135335516
        assert_eq!(SOLUTION.part_2.run_example(0), 117440);
    }

    #[test]
    fn playground() {
        let (_, program) = parse_input(SOLUTION.part_1.task);
        // let (_, program) = parse_input(SOLUTION.part_1.examples[1]);

        let start_reg = 8_u64.pow(program.len() as u32) - 1;
        println!("{start_reg:o}");
        let s = program.iter().join("");
        println!("   {s}");
        let res = traverse(8_u64.pow(program.len() as u32) - 1, &program, 0);
        // 0330553451571124
        // 7777777777777777
        // 2411751543550330

        println!("{res:?}");
    }
}
