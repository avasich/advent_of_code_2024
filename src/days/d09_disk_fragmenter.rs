use std::{cmp::Ordering, collections::BinaryHeap};

use itertools::Itertools;

use crate::utils::{Day, Task, read_lines};

fn p1(filename: &str) -> usize {
    let mut arr = read_lines(filename)
        .next()
        .unwrap()
        .chars()
        .flat_map(|c| c.to_digit(10))
        .enumerate()
        .flat_map(|(i, c)| match i % 2 == 0 {
            true => std::iter::repeat_n(Some(i / 2), c as usize),
            false => std::iter::repeat_n(None, c as usize),
        })
        .collect_vec();

    let mut lt = 0;
    let mut rt = arr.len() - 1;

    loop {
        lt += arr[lt..].iter().take_while(|x| x.is_some()).count();
        rt -= arr[..=rt].iter().rev().take_while(|x| x.is_none()).count();

        if lt >= rt {
            return arr.iter().flatten().enumerate().map(|(i, &x)| i * x).sum();
        }

        arr.swap(lt, rt);
        lt += 1;
        rt -= 1;
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct File {
    id: usize,
    len: usize,
    pos: usize,
}

impl PartialOrd for File {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Self::cmp(self, other))
    }
}

impl Ord for File {
    fn cmp(&self, other: &Self) -> Ordering {
        usize::cmp(&self.pos, &other.pos)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Gap(usize);
impl Ord for Gap {
    fn cmp(&self, other: &Self) -> Ordering {
        usize::cmp(&other.0, &self.0)
    }
}

impl PartialOrd for Gap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Self::cmp(self, other))
    }
}

fn p2_(line: &str) -> usize {
    let (mut files, mut empty, _) =
        line.chars().flat_map(|c| c.to_digit(10)).map(|c| c as usize).enumerate().fold(
            (BinaryHeap::new(), vec![BinaryHeap::new(); 10], 0),
            |(mut files, mut empty, pos), (i, len)| {
                match i % 2 == 0 {
                    true => files.push(File { id: i / 2, pos, len }),
                    false => {
                        empty[len].push(Gap(pos));
                    }
                };

                (files, empty, pos + len)
            },
        );

    let mut res = 0;

    while let Some(f) = files.pop() {
        let gap = empty
            .iter_mut()
            .enumerate()
            .skip(f.len)
            .filter_map(|(gap_len, gaps)| Some((*gaps.peek()?, gap_len, gaps)))
            .filter(|&(gap_pos, _, _)| gap_pos.0 < f.pos)
            .max_by_key(|(gap_pos, ..)| *gap_pos);

        match gap {
            Some((gap_pos, gap_len, gaps)) => {
                files.push(File { id: f.id, pos: gap_pos.0, len: f.len });
                let new_len = gap_len - f.len;
                gaps.pop();
                if new_len > 0 {
                    empty[new_len].push(Gap(gap_pos.0 + f.len));
                }
            }
            None => res += f.id * (2 * f.pos + f.len - 1) * f.len / 2,
        }
    }
    res
}

fn p2(filename: &str) -> usize {
    p2_(&read_lines(filename).next().unwrap())
}

pub const SOLUTION: Day<usize, usize> = Day {
    part_1: Task {
        examples: &["./inputs/day_09/example_1.txt", "./inputs/day_09/example_2.txt"],
        task: "./inputs/day_09/task.txt",
        func: p1,
    },
    part_2: Task {
        examples: &["./inputs/day_09/example_2.txt", "./inputs/day_09/example_3.txt"],
        task: "./inputs/day_09/task.txt",
        func: p2,
    },
};

#[cfg(test)]
mod d09_tests {
    use std::hint::black_box;

    use super::*;

    extern crate test;
    use test::bench::Bencher;

    #[test]
    fn p1_example_test() {
        let res = SOLUTION.part_1.run_example(0);
        assert_eq!(res, 60);
        let res = SOLUTION.part_1.run_example(1);
        assert_eq!(res, 1928);
    }

    #[test]
    fn p2_example_test() {
        // 6408966547049 yay!
        let res = SOLUTION.part_2.run_example(0);
        assert_eq!(res, 2858);

        let res = SOLUTION.part_2.run_example(1);
        assert_eq!(res, 2900);
    }

    #[bench]
    fn p2_1(b: &mut Bencher) {
        let line = read_lines("./inputs/day_09/task.txt").next().unwrap();
        //1426371

        b.iter(|| black_box(p2_(&line)));
    }
}
