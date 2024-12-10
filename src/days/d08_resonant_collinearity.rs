use std::collections::HashMap;

use itertools::Itertools;

use crate::utils::{Day, Task, read_lines};

#[allow(clippy::type_complexity)]
fn parse_file(filename: &str) -> (HashMap<char, Vec<(i32, i32)>>, i32, i32) {
    let (mut w, mut h) = (0, 0);
    let map = read_lines(filename)
        .enumerate()
        .flat_map(|(y, line)| {
            (w, h) = (line.len() as i32, y as i32);
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(|(x, c)| (c, (x as i32, y as i32)))
                .collect::<Vec<_>>()
        })
        .fold(HashMap::<_, Vec<_>>::new(), |mut map, (c, pos)| {
            map.entry(c).or_default().push(pos);
            map
        });

    (map, w, h + 1)
}

fn distinct_pairs<T: Copy>(arr: &[T]) -> impl Iterator<Item = (T, T)> {
    arr.iter().enumerate().flat_map(|(i, &x)| arr[i + 1..].iter().map(move |&y| (x, y)))
}

fn p1(filename: &str) -> usize {
    let (antennas, w, h) = parse_file(filename);
    antennas
        .values()
        .flat_map(|xys| distinct_pairs(xys))
        .flat_map(|((x1, y1), (x2, y2))| [(2 * x1 - x2, 2 * y1 - y2), (2 * x2 - x1, 2 * y2 - y1)])
        .filter(|(x, y)| (0..w).contains(x) && (0..h).contains(y))
        .unique()
        .count()
}

fn p2(filename: &str) -> usize {
    let (antennas, w, h) = parse_file(filename);
    let draw_line = |(x, y), (dx, dy)| {
        std::iter::successors(Some((x, y)), move |&(x, y)| Some((x + dx, y + dy)))
            .take_while(|(x, y)| (0..w).contains(x) && (0..h).contains(y))
    };
    antennas
        .values()
        .flat_map(|xys| distinct_pairs(xys))
        .flat_map(|((x1, y1), (x2, y2))| {
            std::iter::chain(
                draw_line((x1, y1), (x2 - x1, y2 - y1)),
                draw_line((x2, y2), (x1 - x2, y1 - y2)),
            )
        })
        .unique()
        .count()
}

pub const SOLUTION: Day<usize, usize> = Day {
    day: 8,
    part_1: Task { examples: &["example_p1_1.txt", "example_p1_2.txt"], func: p1 },
    part_2: Task { examples: &["example_p2.txt"], func: p2 },
};

#[cfg(test)]
mod d08_tests {
    use super::*;

    #[test]
    fn part_1_example_test() {
        let res = SOLUTION.run_example_1(0);
        assert_eq!(res, 14);

        let res = SOLUTION.run_example_1(1);
        assert_eq!(res, 4);
    }

    #[test]
    fn part_2_example_test() {
        let res = SOLUTION.run_example_2(0);
        assert_eq!(res, 9);
    }
}
