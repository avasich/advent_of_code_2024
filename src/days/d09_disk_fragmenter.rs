use itertools::Itertools;

use crate::utils::{Day, Task, read_lines};

fn _print_arr(arr: &[Option<usize>]) {
    let s = arr.iter().fold(String::new(), |mut s, c| {
        let a = c.map(|c| c.to_string()).unwrap_or(".".to_string());
        s.push_str(&a);
        s.push(' ');
        s
    });
    println!("{s}");
}

fn p1(filename: &str) -> usize {
    let line = read_lines(filename).next().unwrap();

    let mut arr = line
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
            return arr.iter().enumerate().filter_map(|(i, x)| x.map(|x| x * i)).sum();
        }

        arr.swap(lt, rt);
        lt += 1;
        rt -= 1;
    }
}
fn p2(filename: &str) -> u64 {
    0
}

pub const SOLUTION: Day<usize, u64> = Day {
    part_1: Task {
        examples: &["./inputs/day_09/example_1.txt", "./inputs/day_09/example_2.txt"],
        task: "./inputs/day_09/task.txt",
        func: p1,
    },
    part_2: Task {
        examples: &["./inputs/day_09/example_1.txt"],
        task: "./inputs/day_09/task.txt",
        func: p2,
    },
};

#[cfg(test)]
mod d09_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        let res = SOLUTION.part_1.run_example(0);
        assert_eq!(res, 60);
        let res = SOLUTION.part_1.run_example(1);
        assert_eq!(res, 1928);
    }

    #[test]
    fn p2_example_test() {
        let res = SOLUTION.part_2.run_example(0);
        assert_eq!(res, 0);
    }
}
