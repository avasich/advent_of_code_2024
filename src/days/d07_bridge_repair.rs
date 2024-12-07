use std::ops::ControlFlow;

use crate::utils::{Day, Task, read_lines};

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
    std::iter::successors(Some(x), |&v| Some(v / 10))
        .take_while(|&v| v > 0)
        .fold(1, |acc, _| acc * 10)
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
                        .try_fold(xs[0], |acc, (x, op)| match op {
                            _ if acc > target => ControlFlow::Break(()),
                            0 => ControlFlow::Continue(acc + x),
                            1 => ControlFlow::Continue(acc * x),
                            2 => ControlFlow::Continue(acc * next_power_of_ten(x) + x),
                            _ => unreachable!(),
                        })
                        .continue_value()
                })
                .any(|res| res == target)
                .then_some(target)
        })
        .sum()
}

pub const SOLUTION: Day<u64, u64> = Day {
    part_1: Task {
        examples: &["./inputs/day_07/example.txt"],
        task: "./inputs/day_07/task.txt",
        func: p1_two_ops,
    },
    part_2: Task {
        examples: &["./inputs/day_07/example.txt"],
        task: "./inputs/day_07/task.txt",
        func: p2_three_ops,
    },
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
