use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

use crate::{
    day,
    utils::{Day, Task, read_lines},
};

fn parse_input(filename: &str) -> (usize, usize, usize, impl Iterator<Item = (usize, usize)>) {
    let mut lines = read_lines(filename);
    let (h, w, steps) =
        lines.next().unwrap().split(',').flat_map(str::parse).collect_tuple().unwrap();

    let pairs = lines.filter(|line| !line.is_empty()).flat_map(|line| {
        let (a, b) = line.split_once(',')?;
        Some((a.parse::<usize>().ok()?, b.parse::<usize>().ok()?))
    });

    (w, h, steps, pairs)
}

fn route(w: usize, h: usize, blocks: HashSet<(usize, usize)>) -> Option<usize> {
    let mut visited = HashSet::from([(0, 0)]);
    let mut q = VecDeque::from([(0, 0, 0)]);

    while let Some((x, y, step)) = q.pop_front() {
        if (x, y) == (w - 1, h - 1) {
            return Some(step);
        }

        let next_steps = [
            (x > 0).then(|| (x - 1, y, step + 1)),
            (y > 0).then(|| (x, y - 1, step + 1)),
            (x + 1 < w).then_some((x + 1, y, step + 1)),
            (y + 1 < h).then_some((x, y + 1, step + 1)),
        ]
        .into_iter()
        .flatten()
        .filter(|&(x, y, _)| !blocks.contains(&(x, y)) && visited.insert((x, y)));

        q.extend(next_steps);
    }

    None
}

fn p1(filename: &str) -> usize {
    let (w, h, steps, blocks) = parse_input(filename);
    route(w, h, blocks.take(steps).collect()).unwrap()
}

fn p2(filename: &str) -> (usize, usize) {
    let (w, h, _, blocks) = parse_input(filename);
    let blocks: Vec<_> = blocks.collect();

    let (mut l, mut r) = (0, blocks.len());
    while r > l {
        let m = (r + l) / 2;
        match route(w, h, blocks[..=m].iter().copied().collect()) {
            Some(_) => l = m + 1,
            None => r = m,
        }
    }

    blocks[r]
}

pub const SOLUTION: Day<usize, (usize, usize)> = day! { 18,
    part_1: { examples: ["example.txt"], func: p1 },
    part_2: { examples: ["example.txt"], func: p2 }
};

#[cfg(test)]
mod d18_tests {
    use super::*;

    #[test]
    fn p1_examples_test() {
        assert_eq!(SOLUTION.part_1.run_example(0), 22);
    }

    #[test]
    fn p2_examples_test() {
        assert_eq!(SOLUTION.part_2.run_example(0), (6, 1));
    }
}
