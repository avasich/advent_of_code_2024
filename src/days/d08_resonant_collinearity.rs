use std::{
    cmp::Ordering::{Greater, Less},
    collections::HashMap,
};

use itertools::Itertools;

use crate::utils::{Day, Task, read_lines};

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
        .filter(|&(x, y)| x >= 0 && x < w && y >= 0 && y < h)
        .unique()
        .count()
}

fn p2(filename: &str) -> usize {
    let (antennas, w, h) = parse_file(filename);

    let res = antennas
        .iter()
        .flat_map(|(_, xys)| {
            (0..xys.len()).flat_map(move |i| (i + 1..xys.len()).map(move |j| (xys[i], xys[j])))
        })
        .flat_map(|((x1, y1), (x2, y2))| {
            let (dx, dy) = (x2 - x1, y2 - y1);
            let (dx, dy) = (dx.abs(), dy.abs());

            use std::cmp::Ordering::*;
            match (x2.cmp(&x1), y2.cmp(&y1)) {
                (Greater, Greater) | (Less, Less) => {
                    let (kx, ky) = (x1 / dx, y1 / dy);
                    let k = std::cmp::min(kx, ky);
                    let (x0, y0) = (x1 - k * dx, y1 - k * dy);
                    (0..)
                        .map(|i| (x0 + i * dx, y0 + i * dy))
                        .take_while(|&(x, y)| x >= 0 && x < w && y >= 0 && y < h)
                        .collect::<Vec<_>>()
                }
                (Greater, Less) | (Less, Greater) => {
                    let (x, y) = (x1.min(x2), y1.max(y2));
                    let k = x / dx;
                    let x0 = x - k * dx;
                    let y0 = y + k * dy;
                    (0..)
                        .map(|i| (x0 + i * dx, y0 - i * dy))
                        .take_while(|&(_, y)| y >= 0)
                        .filter(|&(x, y)| x >= 0 && x < w && y < h)
                        .collect()
                }

                (Equal, _) => {
                    let y0 = y1 % dy;
                    (0..)
                        .map(|i| (x1, y0 + i * dy))
                        .take_while(|&(x, y)| x >= 0 && x < w && y >= 0 && y < h)
                        .collect()
                }
                (_, Equal) => {
                    let x0 = x1 % dx;
                    (0..)
                        .map(|i| (x0 + i * dx, y1))
                        .take_while(|&(x, y)| x >= 0 && x < w && y >= 0 && y < h)
                        .collect()
                }
            }
            // match (dx, dy) {
            //     (..0, ..0) | (1.., 1..) => {}
            //     (..0, 1..) | (1.., ..0) => {}
            //     (0, _) => {}
            //     (_, 0) => {}
            // }
        })
        // .filter(|&(x, y)| x >= 0 && x < w && y >= 0 && y < h)
        .unique()
        .count();

    res
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
}
