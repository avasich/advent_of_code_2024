use std::collections::HashMap;

use crate::{
    day,
    utils::{Day, Task, read_lines},
};

fn parse_numbers(filename: &str) -> Vec<usize> {
    read_lines(filename).next().unwrap().split_whitespace().flat_map(str::parse::<usize>).collect()
}

fn split_number(n: usize) -> Option<(usize, usize)> {
    std::iter::successors(Some(10), |&x| Some(10 * x))
        .find(|&x| x * x >= n)
        .filter(|&x| x * x <= 10 * n)
        .map(|x| (n / x, n % x))
}

fn req(n: usize, d: usize, t: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    if let Some(&v) = memo.get(&(n, d)) {
        return v;
    }

    if d == t {
        return 1;
    }

    let res = match split_number(n) {
        Some((a, b)) => req(a, d + 1, t, memo) + req(b, d + 1, t, memo),
        None => req((n * 2024).max(1), d + 1, t, memo),
    };

    memo.insert((n, d), res);
    res
}

fn make_blinks(ns: Vec<usize>, blinks: usize) -> usize {
    let mut memo = HashMap::new();
    ns.into_iter().map(|n| req(n, 0, blinks, &mut memo)).sum()
}

fn p1(filename: &str) -> usize {
    make_blinks(parse_numbers(filename), 25)
}
fn p2(filename: &str) -> usize {
    make_blinks(parse_numbers(filename), 75)
}

pub const SOLUTION: Day<usize, usize> = day! { 11,
    part_1: { examples: ["example_2.txt"], func: p1 },
    part_2: { examples: ["example_2.txt"], func: p2 }
};

#[cfg(test)]
mod d11_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        let res = SOLUTION.part_1.run_example(0);
        assert_eq!(res, 55312);
    }

    #[test]
    fn p2_example_test() {}
}

#[cfg(test)]
mod playground {
    #[test]
    fn foo() {}
}
