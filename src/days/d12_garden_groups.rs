use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

use crate::{
    day,
    utils::{Day, Task, read_lines},
};

type Intervals = Vec<(usize, usize)>;

fn extract_area(
    (x0, y0): (usize, usize),
    map: &mut [Vec<Option<char>>],
) -> Option<Vec<(usize, Intervals)>> {
    let c = map[y0][x0]?;
    let (w, h) = (map[0].len(), map.len());
    
    let mut res: HashMap<_, Vec<_>> = HashMap::new();
    let mut q = VecDeque::new();
    q.push_back((x0, y0));

    while let Some((x, y)) = q.pop_front() {
        if map[y][x] != Some(c) {
            continue;
        }
        map[y][x].take();

        res.entry(y).or_default().push(x);

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

    let res: Vec<(_, Vec<_>)> = res
        .into_iter()
        .map(|(y, mut v)| {
            v.sort();

            let mut intervals = vec![];
            let (mut start, mut curr) = (v[0], v[0]);

            for &x in &v[1..] {
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
        .sorted_by_key(|&(y, _)| y)
        .collect();

    Some(res)
}

fn intersection_length(a: &[(usize, usize)], b: &[(usize, usize)]) -> usize {
    let (mut i, mut j) = (0, 0);
    let mut res = 0;

    while i < a.len() && j < b.len() {
        let (start_a, end_a) = a[i];
        let (start_b, end_b) = b[j];

        let start_overlap = start_a.max(start_b);
        let end_overlap = end_a.min(end_b);

        if start_overlap <= end_overlap {
            res += end_overlap - start_overlap + 1;
        }

        match end_a < end_b {
            true => i += 1,
            false => j += 1,
        }
    }

    res
}

fn parse_intervals(filename: &str) -> impl Iterator<Item = Vec<(usize, Intervals)>> {
    let mut map: Vec<Vec<_>> =
        read_lines(filename).map(|line| line.chars().map(Some).collect()).collect();
    let (w, h) = (map[0].len(), map.len());

    (0..h)
        .flat_map(move |y| (0..w).map(move |x| (x, y)))
        .flat_map(move |(x, y)| extract_area((x, y), &mut map))
}

fn p1(filename: &str) -> usize {
    parse_intervals(filename)
        .map(|rows| {
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

/*

0123456789
aaa.aaaaa.
..aaa.....
aaa.aaaaaa.

(0 8)
(2 4)
(0 2) (4 9)

 */

fn p2(filename: &str) -> usize {
    0
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
        let res = SOLUTION.part_1.run_example(0);
        assert_eq!(res, 140);
        let res = SOLUTION.part_1.run_example(1);
        assert_eq!(res, 772);
        let res = SOLUTION.part_1.run_example(2);
        assert_eq!(res, 1930);
    }

    #[test]
    fn p2_example_test() {
        let res = SOLUTION.part_2.run_example(0);
        assert_eq!(res, 80);
        let res = SOLUTION.part_2.run_example(1);
        assert_eq!(res, 436);
        let res = SOLUTION.part_2.run_example(2);
        assert_eq!(res, 236);
        let res = SOLUTION.part_2.run_example(2);
        assert_eq!(res, 368);
    }

    #[test]
    fn playground() {
        p1(SOLUTION.part_1.examples[1]);
    }
}
/*

   0123
 0 AAAA
 1 BBCD
 2 BBCC
 3 EEEC


'A': [(0, 0), (1, 0), (2, 0), (3, 0)],
'B': [(0, 1), (1, 1), (0, 2), (1, 2)],
'C': [(2, 1), (2, 2), (3, 2), (3, 3)],
'D': [(3, 1)]
'E': [(0, 3), (1, 3), (2, 3)],


OOOOO
OOOXO
OOOOO
OXOXO
OOOOO


'O': [
    (0, 0), (1, 0), (2, 0), (3, 0), (4, 0)
    (0, 1), (1, 1), (2, 1),         (4, 1),
    (0, 2), (1, 2), (2, 2), (3, 2), (4, 2)
    (0, 3),
    (0, 4),
    (2, 3), (1, 4), (2, 4), (4, 3), (3, 4), (4, 4)
    ],
'X': [(3, 3)]

0: (0 4)
1: (0 2) (4 4)

(0, (0 4))
(1, (0 2))
(1, (4 4))




(0, (2 3))


(1, (4 4))



{
'X': [(3, 3)],

'O': [
(0, 0), (1, 0), (2, 0), (3, 0), (4, 0),
(0, 1), (2, 1), (4, 1),
(0, 2), (1, 2), (2, 2), (3, 2), (4, 2),
(0, 3), (2, 3), (4, 3),
(0, 4), (1, 4), (2, 4), (3, 4), (4, 4)]}


 */
