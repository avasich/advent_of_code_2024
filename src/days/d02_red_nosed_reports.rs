use Kind::*;

use crate::{
    day,
    utils::{Day, Task, read_lines},
};

#[derive(Copy, Clone, Debug)]
enum Kind {
    Dec,
    Inc,
    Unsafe,
}

impl Kind {
    fn new(a: i32, b: i32) -> Self {
        Self::from_diff(b - a)
    }

    fn from_diff(diff: i32) -> Self {
        #[allow(non_contiguous_range_endpoints)]
        match diff {
            -3..0 => Dec,
            1..=3 => Inc,
            _ => Unsafe,
        }
    }
}

fn p1_count_safe(filename: &str) -> usize {
    read_lines(filename)
        .map(|line| find_unsafe(line.split_whitespace().flat_map(str::parse::<i32>)).is_none())
        .filter(|&x| x)
        .count()
}

fn p2_count_safe_allow_error(filename: &str) -> usize {
    read_lines(filename)
        .map(|line| line.split_whitespace().flat_map(str::parse::<i32>).collect::<Vec<_>>())
        .map(|v| with_error_allowed(&v, None))
        .filter(|&x| x)
        .count()
}

fn find_unsafe(xs: impl Iterator<Item = i32>) -> Option<usize> {
    xs.map_windows(|&[a, b]| Kind::new(a, b))
        .map_windows(|[s1, s2]| !matches!([s1, s2], [Inc, Inc] | [Dec, Dec]))
        .enumerate()
        .find_map(|(i, err)| err.then_some(i))
}

fn with_error_allowed(xs: &[i32], skip: Option<usize>) -> bool {
    match skip {
        // Some(skip) => {
        //     let start = skip.saturating_sub(2);
        //     find_unsafe(std::iter::chain(&xs[start..skip], &xs[skip + 1..]).copied()).is_none()
        // }

        // inputs are short, so that's ok too
        Some(skip) => {
            find_unsafe(std::iter::chain(&xs[..skip], &xs[skip + 1..]).copied()).is_none()
        }

        None => match find_unsafe(xs.iter().copied()) {
            None => true,
            Some(i) => (i..i + 3).any(|j| with_error_allowed(xs, Some(j))),
        },
    }
}

pub const SOLUTION: Day<usize, usize> = day! { 2,
    part_1: { examples: ["example.txt"], func: p1_count_safe },
    part_2: { examples: ["example.txt"], func: p2_count_safe_allow_error }
};

#[cfg(test)]
mod d02_playground {

    #[test]
    #[ignore]
    fn playground() {}
}

#[cfg(test)]
mod d02_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        let res = SOLUTION.part_1.run_example(0);
        assert_eq!(res, 2);
    }

    #[test]
    fn p2_example_test() {
        let res = SOLUTION.part_2.run_example(0);
        assert_eq!(res, 4);
    }
}
