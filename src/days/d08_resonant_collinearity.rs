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

fn p1(filename: &str) -> usize {
    let (antennas, w, h) = parse_file(filename);

    antennas
        .iter()
        .flat_map(|(_, xys)| {
            (0..xys.len()).flat_map(move |i| (i + 1..xys.len()).map(move |j| (xys[i], xys[j])))
        })
        .flat_map(|((x1, y1), (x2, y2))| [(2 * x1 - x2, 2 * y1 - y2), (2 * x2 - x1, 2 * y2 - y1)])
        .filter(|(x, y)| (0..w).contains(x) && (0..h).contains(y))
        .unique()
        .count()
}

fn p2(filename: &str) -> usize {
    let (antennas, w, h) = parse_file(filename);

    antennas
        .iter()
        .flat_map(|(_, xys)| {
            (0..xys.len()).flat_map(move |i| (i + 1..xys.len()).map(move |j| (xys[i], xys[j])))
        })
        .flat_map(|((x1, y1), (x2, y2))| {
            use std::cmp::Ordering::*;

            fn div(a: i32, b: i32) -> i32 {
                if a % b == 0 { a / b - 1 } else { a / b }
            }

            let (dx, dy) = ((x2 - x1).abs(), (y2 - y1).abs());

            let (x0, y0, dx, dy, k) = match (x2.cmp(&x1), y2.cmp(&y1)) {
                (Equal, _) => {
                    let (x0, y0) = (x1, y1 % dy);
                    let k = div(h - y0, dy);
                    (x0, y0, dx, dy, k)
                }
                (_, Equal) => {
                    let (x0, y0) = (x1 % dx, y1);
                    let k = div(w - x0, dx);
                    (x0, y0, dx, dy, k)
                }
                (Greater, _) => {
                    let i = std::cmp::min(x1 / dx, y1 / dy);
                    let (x0, y0) = (x1 - i * dx, y1 - i * dy);
                    let k = std::cmp::min(div(w - x0, dx), div(h - y0, dy));
                    (x0, y0, dx, dy, k)
                }
                (Less, _) => {
                    let i = std::cmp::min(x2 / dx, div(h - y2, dy));
                    let (x0, y0) = (x2 - i * dx, y2 + i * dy);
                    let k = std::cmp::min(div(w - x0, dx), y0 / dy);
                    (x0, y0, dx, -dy, k)
                }
            };

            std::iter::successors(Some((x0, y0)), move |&(x, y)| Some((x + dx, y + dy)))
                .take(k as usize + 1)
        })
        .unique()
        .count()
}

pub const SOLUTION: Day<usize, usize> = Day {
    part_1: Task {
        examples: &["./inputs/day_08/example_p1_1.txt", "./inputs/day_08/example_p1_2.txt"],
        task: "./inputs/day_08/task.txt",
        func: p1,
    },
    part_2: Task {
        examples: &["./inputs/day_08/example_p2.txt"],
        task: "./inputs/day_08/task.txt",
        func: p2,
    },
};

#[cfg(test)]
mod d08_tests {
    use super::*;

    #[test]
    fn part_1_example_test() {
        let res = SOLUTION.part_1.run_example(0);
        assert_eq!(res, 14);

        let res = SOLUTION.part_1.run_example(1);
        assert_eq!(res, 4);
    }

    #[test]
    fn part_2_example_test() {
        let res = SOLUTION.part_2.run_example(0);
        assert_eq!(res, 9);
    }

    #[test]
    fn foo() {
        let m = 99;
        let a = 110 % m;
        let b = 110 % -m;

        println!("{a} {b}")
    }
}
