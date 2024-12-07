use crate::utils::{Day, Task, read_lines};

fn p1(filename: &str) -> u64 {
    read_lines(filename)
        .flat_map(|line| {
            let (v, xs) = line.split_once(':')?;
            let target: u64 = v.parse().ok()?;
            let xs: Vec<u64> = xs.trim().split_whitespace().flat_map(str::parse).collect();
            Some((target, xs))
        })
        .filter_map(|(target, xs)| {
            let gaps = xs.len() as u32 - 1;
            (0..2u64.pow(gaps))
                .map(|x| {
                    (0..gaps).map(move |pos| x & (1 << pos) != 0).enumerate().fold(
                        xs[0],
                        |acc, (i, op)| match op {
                            true => acc * xs[i + 1],
                            false => acc + xs[i + 1],
                        },
                    )
                })
                .any(|res| res == target)
                .then_some(target)
        })
        .sum()
}

fn p2(filename: &str) -> usize {
    0
}

pub const SOLUTION: Day<u64, usize> = Day {
    part_1: Task {
        examples: &["./inputs/day_07/example.txt"],
        task: "./inputs/day_07/task.txt",
        func: p1,
    },
    part_2: Task {
        examples: &["./inputs/day_06/example.txt"],
        task: "./inputs/day_06/task.txt",
        func: p2,
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
        assert_eq!(res, 6);
    }

    #[test]
    fn foo() {}
}
