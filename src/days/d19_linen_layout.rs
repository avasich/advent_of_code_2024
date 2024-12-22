use std::collections::HashSet;

use crate::{
    day,
    utils::{Day, Task, read_lines},
};

fn parse_input(filename: &str) -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    let mut lines = read_lines(filename);

    let towels = lines.next().unwrap().split(", ").map(|p| p.chars().collect()).collect();
    lines.next();
    let patterns = lines.map(|line| line.chars().collect()).collect();

    (towels, patterns)
}

fn check(towels: &HashSet<&[char]>, pattern: &[char]) -> bool {
    (0..pattern.len()).rev().map(|i| (i, &pattern[i..])).fold(
        vec![false; pattern.len()],
        |mut flags, (i, suffix)| {
            flags[i] = (0..suffix.len())
                .filter(|&j| towels.contains(&suffix[..=j]))
                .any(|j| i + j + 1 >= pattern.len() || flags[i + j + 1]);
            flags
        },
    )[0]
}

fn p1(filename: &str) -> usize {
    let (towels, patterns) = parse_input(filename);
    let towels: HashSet<_> = towels.iter().map(Vec::as_slice).collect();

    patterns.iter().filter(|p| check(&towels, p.as_slice())).count()
}

fn p2(_filename: &str) -> usize {
    0
}

pub const SOLUTION: Day<usize, usize> = day! { 19,
    part_1: { examples: ["example.txt"], func: p1 },
    part_2: { examples: ["example.txt"], func: p2 }
};

#[cfg(test)]
mod d19_tests {
    use super::*;

    #[test]
    fn p1_examples_test() {
        assert_eq!(SOLUTION.part_1.run_example(0), 6);
    }

    #[test]
    fn p2_examples_test() {
        assert_eq!(SOLUTION.part_2.run_example(0), 0);
    }

    #[test]
    fn playground() {}
}
