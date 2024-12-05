use std::{
    collections::{HashMap, HashSet},
    ops::ControlFlow,
};

use crate::utils::{Day, Task, read_lines};

fn p1(filename: &str) -> u32 {
    let mut lines = read_lines(filename);

    let afters = std::iter::from_fn(|| lines.next().filter(|line| !line.is_empty()))
        .flat_map(|line| {
            let (before, after) = line.split_once('|')?;
            Some((before.parse::<u32>().ok()?, after.parse::<u32>().ok()?))
        })
        .fold(HashMap::new(), |mut afters, (before, after)| {
            afters.entry(before).or_insert(vec![]).push(after);
            afters
        });

    let check_found = |n: u32, found: &HashSet<u32>| {
        match afters.get(&n) {
            None => true,
            Some(should_be_after) => !should_be_after.iter().any(|a| found.contains(a)),
        }
    };

    lines
        .flat_map(|line| {
            let nums: Vec<_> = line.split(',').flat_map(str::parse::<u32>).collect();
            nums.iter()
                .try_fold(HashSet::new(), |mut found, &n| {
                    if check_found(n, &found) {
                        found.insert(n);
                        ControlFlow::Continue(found)
                    } else {
                        ControlFlow::Break(())
                    }
                })
                .is_continue()
                .then_some(nums[nums.len() / 2])
        })
        .sum()
}

fn p2(filname: &str) -> u32 {
    0
}

pub const SOLUTION: Day<u32, u32> = Day {
    part_1: Task {
        examples: &["./inputs/day_05/example.txt"],
        task: "./inputs/day_05/task.txt",
        func: p1,
    },
    part_2: Task {
        examples: &["./inputs/day_05/example.txt"],
        task: "./inputs/day_05/task.txt",
        func: p2,
    },
};

#[cfg(test)]
mod d05_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        let res = SOLUTION.part_1.run_example(0);
        assert_eq!(res, 143);
    }

    #[test]
    fn p2_example_test() {
        let _res = SOLUTION.part_1.run_example(0);
    }
}
