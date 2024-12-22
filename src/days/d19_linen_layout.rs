use std::{collections::HashSet, str::pattern::Pattern};

use crate::{
    day,
    utils::{Day, Task, read_lines},
};

fn parse_input(filename: &str) -> (Vec<String>, Vec<String>) {
    let mut lines = read_lines(filename);

    let towels = lines.next().unwrap().split(", ").map(String::from).collect();
    lines.next();
    let patterns = lines.collect();

    (towels, patterns)
}

fn ways_to_combine(towels: &HashSet<&str>, pattern: &str) -> usize {
    (0..pattern.len()).map(|i| (i, &pattern[..=i])).fold(
        vec![0_usize; pattern.len()],
        |mut flags, (i, prefix)| {
            towels.iter().filter(|t| t.is_suffix_of(prefix)).for_each(|t| {
                flags[i] += if i >= t.len() { flags[i - t.len()] } else { 1 };
            });
            flags
        },
    )[pattern.len() - 1]
}

fn p1(filename: &str) -> usize {
    let (towels, patterns) = parse_input(filename);
    let towels: HashSet<_> = towels.iter().map(String::as_str).collect();

    patterns.iter().filter(|p| ways_to_combine(&towels, p.as_str()) > 0).count()
}

fn p2(filename: &str) -> usize {
    let (towels, patterns) = parse_input(filename);
    let towels: HashSet<_> = towels.iter().map(String::as_str).collect();

    patterns.iter().map(|p| ways_to_combine(&towels, p.as_str())).sum()
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
