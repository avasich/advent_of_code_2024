use crate::{
    day,
    utils::{Day, Task, read_lines},
};

fn parse_file(filename: &str) -> impl Iterator<Item = (u64, Vec<u64>)> {
    read_lines(filename).flat_map(|line| {
        let (v, xs) = line.split_once(':')?;
        let target: u64 = v.parse().ok()?;
        let xs: Vec<u64> = xs.split_whitespace().flat_map(str::parse).collect();
        Some((target, xs))
    })
}

fn p1_two_ops(filename: &str) -> u64 {
    parse_file(filename)
        .filter_map(|(target, xs)| {
            let gaps = xs.len() - 1;
            (0..2u64.pow(gaps as u32))
                .map(|ops| {
                    (0..gaps).fold(xs[0], |acc, i| match ops & (1 << i) {
                        0 => acc + xs[i + 1],
                        _ => acc * xs[i + 1],
                    })
                })
                .any(|res| res == target)
                .then_some(target)
        })
        .sum()
}

#[inline(always)]
fn next_power_of_ten(x: u64) -> u64 {
    std::iter::successors(Some(10), |&v| (v <= x).then_some(10 * v)).last().unwrap()
}

fn p2_three_ops(filename: &str) -> u64 {
    parse_file(filename)
        .filter_map(|(target, xs)| {
            let gaps = xs.len() - 1;
            (0..3u64.pow(gaps as u32))
                .flat_map(|ops| {
                    std::iter::successors(Some(ops), |&v| Some(v / 3))
                        .take(gaps)
                        .enumerate()
                        .map(|(i, op)| (xs[i + 1], op % 3))
                        // since all numbers are strictly greater than 0
                        // and all ops increase the result
                        .try_fold(xs[0], |acc, (x, op)| match op {
                            _ if acc > target => None,
                            0 => Some(acc + x),
                            1 => Some(acc * x),
                            2 => Some(acc * next_power_of_ten(x) + x),
                            _ => unreachable!(),
                        })
                })
                .any(|res| res == target)
                .then_some(target)
        })
        .sum()
}

pub const SOLUTION: Day<u64, u64> = day! { 7,
    part_1: { examples: ["example.txt"], func: p1_two_ops },
    part_2: { examples: ["example.txt"], func: p2_three_ops }
};

#[cfg(test)]
mod d07_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        let res = SOLUTION.part_1.run_example(0);
        assert_eq!(res, 3749);
    }

    #[test]
    fn p2_example_test() {
        let res = SOLUTION.part_2.run_example(0);
        assert_eq!(res, 11387);
    }
}
