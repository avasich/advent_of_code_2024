use crate::{
    day,
    utils::{Day, Task, read_lines},
};

fn parse_mul(s: &str, l_br: usize) -> Option<u32> {
    let r_br_max = (l_br + 8).min(s.len() - 1);
    let r_br = s[l_br + 1..=r_br_max].find(')')?;
    let (a, b) = s[l_br + 1..=l_br + r_br].split_once(',')?;
    let a = a.parse::<u32>().ok()?;
    let b = b.parse::<u32>().ok()?;
    Some(a * b)
}

fn p1_sum_of_mul(ss: impl Iterator<Item = String>) -> u32 {
    ss.map(|line| {
        std::iter::successors(Some((line.as_str(), None)), |&(suffix, _)| {
            let l_br = suffix.find("mul(")? + 3;
            Some((&suffix[l_br + 1..], parse_mul(suffix, l_br)))
        })
        .flat_map(|(_, x)| x)
        .sum::<u32>()
    })
    .sum()
}

fn p2_sum_of_mul_enable(ss: impl Iterator<Item = String>) -> u32 {
    ss.scan(true, |enabled, line| {
        let res = (0..line.len())
            .flat_map(|i| {
                if line[i..].starts_with("do()") {
                    *enabled = true;
                    None
                } else if line[i..].starts_with("don't") {
                    *enabled = false;
                    None
                } else if *enabled && line[i..].starts_with("mul(") {
                    parse_mul(&line[i..], 3)
                } else {
                    None
                }
            })
            .sum::<u32>();
        Some(res)
    })
    .sum()
}

fn p1(filename: &str) -> u32 {
    p1_sum_of_mul(read_lines(filename))
}

fn p2(filename: &str) -> u32 {
    p2_sum_of_mul_enable(read_lines(filename))
}

pub const SOLUTION: Day<u32, u32> = day! { 3,
    part_1: { examples: ["example_1.txt"], func: p1 },
    part_2: { examples: ["example_2.txt"], func: p2 }
};

#[cfg(test)]
mod d03_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        let res = SOLUTION.part_1.run_example(0);
        assert_eq!(res, 161);
    }

    #[test]
    fn p2_example_test() {
        let res = SOLUTION.part_2.run_example(0);
        assert_eq!(res, 48);
    }

    #[test]
    fn foo() {}
}
