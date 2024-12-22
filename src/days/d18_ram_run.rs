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

    // let (w, h) = pairs.next().unwrap();

    (w, h, steps, pairs)
}

fn p1(filename: &str) -> Option<usize> {
    let (w, h, steps, blocks) = parse_input(filename);
    let blocks: HashSet<_> = blocks.take(steps).collect();

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

fn p2(filename: &str) -> u64 {
    0
}

pub const SOLUTION: Day<Option<usize>, u64> = day! { 18,
    part_1: { examples: ["example.txt"], func: p1 },
    part_2: { examples: ["example.txt"], func: p2 }
};

#[cfg(test)]
mod d19_tests {
    use super::*;

    #[test]
    fn p1_examples_test() {
        assert_eq!(SOLUTION.part_1.run_example(0), Some(22));
    }
}
