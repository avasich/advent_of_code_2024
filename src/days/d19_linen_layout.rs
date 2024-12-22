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

fn check(towels: &HashSet<&[char]>, pattern: &[char]) -> usize {
    (0..pattern.len()).rev().map(|i| (i, &pattern[i..])).fold(
        vec![0; pattern.len()],
        |mut flags, (i, suffix)| {
            flags[i] = (0..suffix.len() - 1)
                .filter(|&j| towels.contains(&suffix[..=j]))
                .map(|j| flags[i + j + 1])
                .sum::<usize>()
                + if towels.contains(suffix) { 1 } else { 0 };
            flags
        },
    )[0]
}

fn p1(filename: &str) -> usize {
    let (towels, patterns) = parse_input(filename);
    let towels: HashSet<_> = towels.iter().map(Vec::as_slice).collect();

    patterns.iter().filter(|p| check(&towels, p.as_slice()) > 0).count()
}

fn p2(filename: &str) -> usize {
    let (towels, patterns) = parse_input(filename);
    let towels: HashSet<_> = towels.iter().map(Vec::as_slice).collect();

    patterns.iter().map(|p| check(&towels, p.as_slice())).sum()
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
        assert_eq!(SOLUTION.part_2.run_example(0), 16);
    }

    #[test]
    fn playground() {}
}
