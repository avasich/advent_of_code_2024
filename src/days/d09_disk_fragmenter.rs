use std::collections::BTreeSet;

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

// len id
// type Block = (usize, usize);

// start len id
type Files = BTreeSet<(usize, usize, usize)>;
// len - [positions]
type EmptyBlocks = Vec<BTreeSet<usize>>;

fn p2_(line: &str) -> usize {
    let (mut files, mut empty, _) =
        line.chars().flat_map(|c| c.to_digit(10)).map(|c| c as usize).enumerate().fold(
            (BTreeSet::new(), vec![BTreeSet::new(); 10], 0),
            |(mut files, mut empty, pos): (Files, EmptyBlocks, _), (i, len)| {
                match i % 2 == 0 {
                    true => files.insert((pos, len, i / 2)),
                    false => empty[len].insert(pos),
                };

                (files, empty, pos + len)
            },
        );

    let mut res = 0;

    while let Some(f) = files.iter().last().copied() {
        let (file_pos, file_len, id) = f;

        let gap = empty
            .iter_mut()
            .enumerate()
            .skip(file_len)
            .filter_map(|(gap_len, gaps)| Some((*gaps.first()?, gap_len, gaps)))
            .filter(|&(gap_pos, _, _)| gap_pos < file_pos)
            .min();

        match gap {
            Some((gap_pos, gap_len, gaps)) => {
                files.remove(&f);
                gaps.pop_first();
                files.insert((gap_pos, file_len, id));
                let new_len = gap_len - file_len;
                if new_len > 0 {
                    empty[gap_len - file_len].insert(gap_pos + file_len);
                }
            }
            None => {
                res += id * (2 * file_pos + file_len - 1) * file_len / 2;
                files.pop_last();
            }
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
        //2645688

        b.iter(|| black_box(p2_(&line)));
    }
}
