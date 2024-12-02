#![allow(non_contiguous_range_endpoints)]

use crate::utils::{Day, Task, read_lines};

#[derive(Copy, Clone)]
enum Kind {
    Inc,
    Dec,
    Unsafe,
}

fn p1_count_safe(filename: &str) -> usize {
    use Kind::*;

    read_lines(filename)
        .map(|line| {
            line.split_whitespace()
                .flat_map(str::parse::<i32>)
                .map_windows(|[a, b]| match b - a {
                    -3..0 => Dec,
                    1..=3 => Inc,
                    _ => Unsafe,
                })
                .map_windows(|[s1, s2]| matches!([s1, s2], [Inc, Inc] | [Dec, Dec]))
                .all(|x| x)
        })
        .filter(|&x| x)
        .count()
}

fn p2(filename: &str) -> usize {
    todo!()
}

pub const SOLUTION: Day<usize, usize> = Day {
    part_1: Task {
        examples: &["./inputs/day_02/example.txt"],
        task: "./inputs/day_02/task.txt",
        func: p1_count_safe,
    },
    part_2: Task {
        examples: &["./inputs/day_02/example.txt"],
        task: "./inputs/day_02/task.txt",
        func: p2,
    },
};

#[cfg(test)]
mod d02_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        let res = SOLUTION.part_1.run_example(0);
        assert_eq!(res, 2);
    }
}
