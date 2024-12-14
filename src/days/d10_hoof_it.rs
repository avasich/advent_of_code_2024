use std::collections::VecDeque;

use crate::{
    day,
    utils::{Day, Task, read_lines},
};

fn trailhead_score((x0, y0): (usize, usize), map: &[Vec<i32>]) -> usize {
    if map[y0][x0] != 0 {
        return 0;
    }

    let w = map[0].len();
    let h = map.len();

    let mut visited = vec![vec![false; w]; h];
    let mut q = VecDeque::from([(x0, y0, 0)]);
    let mut res = 0;

    while let Some((x, y, e)) = q.pop_front() {
        if visited[y][x] || map[y][x] != e {
            continue;
        }
        visited[y][x] = true;

        if map[y][x] == 9 {
            res += 1;
            continue;
        }

        if x > 0 {
            q.push_back((x - 1, y, e + 1));
        }
        if x + 1 < w {
            q.push_back((x + 1, y, e + 1));
        }
        if y > 0 {
            q.push_back((x, y - 1, e + 1));
        }
        if y + 1 < h {
            q.push_back((x, y + 1, e + 1));
        }
    }

    res
}

fn trailhead_rating((x0, y0): (usize, usize), map: &[Vec<i32>]) -> usize {
    if map[y0][x0] != 0 {
        return 0;
    }

    let w = map[0].len();
    let h = map.len();

    let mut stack = vec![(x0, y0, 0)];
    let mut res = 0;

    while let Some((x, y, e)) = stack.pop() {
        if map[y][x] != e {
            continue;
        }

        if map[y][x] == 9 {
            res += 1;
            continue;
        }

        if x > 0 {
            stack.push((x - 1, y, e + 1));
        }
        if x + 1 < w {
            stack.push((x + 1, y, e + 1));
        }
        if y > 0 {
            stack.push((x, y - 1, e + 1));
        }
        if y + 1 < h {
            stack.push((x, y + 1, e + 1));
        }
    }

    res
}

fn parse_map(filename: &str) -> Vec<Vec<i32>> {
    read_lines(filename)
        .map(|line| line.chars().map(|c| c.to_digit(10).map(|c| c as i32).unwrap_or(-1)).collect())
        .collect()
}

fn p1(filename: &str) -> usize {
    let map = parse_map(filename);
    let w = map[0].len();
    let h = map.len();

    (0..h).flat_map(|y| (0..w).map(move |x| (x, y))).map(|xy| trailhead_score(xy, &map)).sum()
}

fn p2(filename: &str) -> usize {
    let map = parse_map(filename);
    let w = map[0].len();
    let h = map.len();

    (0..h).flat_map(|y| (0..w).map(move |x| (x, y))).map(|xy| trailhead_rating(xy, &map)).sum()
}

pub const SOLUTION: Day<usize, usize> = day! { 10,
    part_1: {
        examples: [
            "example_p1_1.txt",
            "example_p1_2.txt",
            "example_p1_3.txt",
            "example_p1_4.txt",
            "example_p1_5.txt",
        ],
        func: p1,
    },
    part_2: {
        examples: [
            "example_p2_1.txt",
            "example_p2_2.txt",
            "example_p2_3.txt",
            "example_p2_4.txt",
        ],
        func: p2,
    },
};

#[cfg(test)]
mod d10_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        let res = SOLUTION.part_1.run_example(0);
        assert_eq!(res, 1);
        let res = SOLUTION.part_1.run_example(1);
        assert_eq!(res, 2);
        let res = SOLUTION.part_1.run_example(2);
        assert_eq!(res, 4);
        let res = SOLUTION.part_1.run_example(3);
        assert_eq!(res, 3);
        let res = SOLUTION.part_1.run_example(4);
        assert_eq!(res, 36);
    }

    #[test]
    fn p2_example_test() {
        let res = SOLUTION.part_2.run_example(0);
        assert_eq!(res, 3);
        let res = SOLUTION.part_2.run_example(1);
        assert_eq!(res, 13);
        let res = SOLUTION.part_2.run_example(2);
        assert_eq!(res, 227);
        let res = SOLUTION.part_2.run_example(3);
        assert_eq!(res, 81);
    }
}
