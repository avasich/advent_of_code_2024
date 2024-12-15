use std::collections::VecDeque;

use itertools::Itertools;

use crate::{
    day,
    utils::{read_lines, Day, Task},
};

type Intervals = Vec<(usize, usize)>;

fn extract_area(
    (x0, y0): (usize, usize),
    map: &mut [Vec<Option<char>>],
) -> Option<Vec<(usize, Intervals)>> {
    let c = map[y0][x0]?;
    let (w, h) = (map[0].len(), map.len());

    let mut points = vec![];
    let mut q = VecDeque::new();
    q.push_back((x0, y0));

    while let Some((x, y)) = q.pop_front() {
        if map[y][x] != Some(c) {
            continue;
        }
        map[y][x].take();

        points.push((y, x));

        if x > 0 {
            q.push_back((x - 1, y));
        }
        if x + 1 < w {
            q.push_back((x + 1, y));
        }
        if y > 0 {
            q.push_back((x, y - 1));
        }
        if y + 1 < h {
            q.push_back((x, y + 1));
        }
    }

    points.sort();

    let res = std::iter::successors(Some(0), |&start| {
        match points[start..].iter().find_position(|&&(y, _)| points[start].0 != y) {
            _ if start >= points.len() => None,
            None => Some(points.len()),
            Some((i, _)) => Some(start + i),
        }
    })
    .tuple_windows()
    .map(|(i, j)| &points[i..j])
    .map(|arr| {
        let (y, x0) = arr[0];
        let mut intervals = vec![];
        let (mut start, mut curr) = (x0, x0);

        for &(_, x) in &arr[1..] {
            match x == curr + 1 {
                true => curr += 1,
                false => {
                    intervals.push((start, curr));
                    (start, curr) = (x, x);
                }
            }
        }

        intervals.push((start, curr));
        (y, intervals)
    })
    .collect();

    Some(res)
}

fn intersection_length(a: &[(usize, usize)], b: &[(usize, usize)]) -> usize {
    let (mut i, mut j) = (0, 0);
    let mut res = 0;

    while i < a.len() && j < b.len() {
        let (start_a, end_a) = a[i];
        let (start_b, end_b) = b[j];

        let start = start_a.max(start_b);
        let end = end_a.min(end_b);

        if start <= end {
            res += end - start + 1;
        }

        match end_a < end_b {
            true => i += 1,
            false => j += 1,
        }
    }

    res
}

fn sides_change(a: &[(usize, usize)], b: &[(usize, usize)]) -> usize {
    let (mut i, mut j) = (0, 0);
    let mut res = 0;

    while i < a.len() && j < b.len() {
        let (start_a, end_a) = a[i];
        let (start_b, end_b) = b[j];

        use std::cmp::Ordering;
        match (start_a.cmp(&start_b), end_a.cmp(&end_b)) {
            (Ordering::Equal, Ordering::Equal) => {
                // ...aaaaaaa...
                // ...bbbbbbb...
                res += 4;
            }
            (Ordering::Equal, _) | (_, Ordering::Equal) => {
                //  ..aaaaaa.......  ..aaaaaaaaaaa..
                //  ..bbbbbbbbbbb..  ..bbbbbb.......

                //  .......aaaaaa..   ..aaaaaaaaaaa..
                //  ..bbbbbbbbbbb..   .......bbbbbb..
                res += 2;
            }
            _ => {}
        }

        match end_a < end_b {
            true => i += 1,
            false => j += 1,
        }
    }

    res
}

fn parse_intervals(filename: &str) -> impl Iterator<Item = (char, Vec<(usize, Intervals)>)> {
    let mut map: Vec<Vec<_>> =
        read_lines(filename).map(|line| line.chars().map(Some).collect()).collect();
    let (w, h) = (map[0].len(), map.len());

    (0..h)
        .flat_map(move |y| (0..w).map(move |x| (x, y)))
        .flat_map(move |(x, y)| Some((map[y][x]?, extract_area((x, y), &mut map)?)))
}

fn p1(filename: &str) -> usize {
    parse_intervals(filename)
        .map(|(_, rows)| {
            let (area, perimeter) = rows
                .iter()
                .flat_map(|(_, intervals)| intervals)
                .map(|&(start, end)| {
                    let a = end - start + 1;
                    let p = 2 * a + 2;
                    (a, p)
                })
                .fold((0, 0), |(a1, p1), (a2, p2)| (a1 + a2, p1 + p2));

            let intersection = rows
                .array_windows()
                .map(|[(_, r1), (_, r2)]| intersection_length(r1, r2))
                .sum::<usize>();

            let perimeter = perimeter - 2 * intersection;
            perimeter * area
        })
        .sum()
}

fn p2(filename: &str) -> usize {
    parse_intervals(filename)
        .map(|(_, rows)| {
            let (area, sides) = rows
                .iter()
                .flat_map(|(_, intervals)| intervals)
                .map(|&(start, end)| (end - start + 1, 4))
                .fold((0, 0), |(a1, p1), (a2, p2)| (a1 + a2, p1 + p2));

            let diff =
                rows.array_windows().map(|[(_, r1), (_, r2)]| sides_change(r1, r2)).sum::<usize>();

            let total_sides = sides - diff;
            total_sides * area
        })
        .sum()
}

pub const SOLUTION: Day<usize, usize> = day! { 12,
    part_1: {
        examples: [
            "example_1.txt",
            "example_2.txt",
            "example_3.txt",
        ],
        func: p1,
    },
    part_2: {
        examples: [
            "example_1.txt",
            "example_2.txt",
            "example_4.txt",
            "example_5.txt",
        ],
        func: p2,
    }
};

#[cfg(test)]
mod d12_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        assert_eq!(SOLUTION.part_1.run_example(0), 140);
        assert_eq!(SOLUTION.part_1.run_example(1), 772);
        assert_eq!(SOLUTION.part_1.run_example(2), 1930);
    }

    #[test]
    fn p2_example_test() {
        assert_eq!(SOLUTION.part_2.run_example(0), 80);
        assert_eq!(SOLUTION.part_2.run_example(1), 436);
        assert_eq!(SOLUTION.part_2.run_example(2), 236);
        assert_eq!(SOLUTION.part_2.run_example(3), 368);
    }

    #[test]
    fn playground() {}
}
