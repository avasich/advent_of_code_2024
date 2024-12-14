use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use crate::{
    day,
    utils::{Day, Task, read_lines},
};

fn check_nums(nums: &[u32], cmp: &HashMap<(u32, u32), Ordering>) -> bool {
    let mut found = HashSet::new();
    for &rhs in nums {
        let is_ok =
            found.iter().all(|&lhs| !matches!(cmp.get(&(lhs, rhs)), Some(Ordering::Greater)));
        if is_ok {
            found.insert(rhs);
        } else {
            return false;
        }
    }
    true
}

fn cmp_map(ss: &mut impl Iterator<Item = String>) -> HashMap<(u32, u32), Ordering> {
    std::iter::from_fn(|| ss.next().filter(|line| !line.is_empty()))
        .flat_map(|s| {
            let (lhs, rhs) = s.split_once('|')?;
            Some((lhs.parse::<u32>().ok()?, rhs.parse::<u32>().ok()?))
        })
        .fold(HashMap::new(), |mut cmp, (lhs, rhs)| {
            cmp.insert((lhs, rhs), Ordering::Less);
            cmp.insert((rhs, lhs), Ordering::Greater);
            cmp
        })
}

fn p1(filename: &str) -> u32 {
    let mut ss = read_lines(filename);
    let cmp = cmp_map(&mut ss);

    ss.map(|line| line.split(',').flat_map(str::parse::<u32>).collect::<Vec<_>>())
        .filter(|nums| check_nums(nums, &cmp))
        .map(|nums| nums[nums.len() / 2])
        .sum()
}

fn p2(filename: &str) -> u32 {
    let mut ss = read_lines(filename);
    let cmp = cmp_map(&mut ss);

    ss.map(|line| line.split(',').flat_map(str::parse::<u32>).collect::<Vec<_>>())
        .filter(|nums| !check_nums(nums, &cmp))
        .map(|mut nums| {
            nums.sort_by(|&a, &b| *cmp.get(&(a, b)).unwrap_or(&Ordering::Equal));
            nums[nums.len() / 2]
        })
        .sum()
}

pub const SOLUTION: Day<u32, u32> = day! { 5,
    part_1: { examples: ["example.txt"], func: p1 },
    part_2: { examples: ["example.txt"], func: p2 }
};

#[cfg(test)]
mod d05_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        let res = SOLUTION.part_1.run_example(0);
        assert_eq!(res, 143);
    }

    #[test]
    fn p2_example_test() {
        let res = SOLUTION.part_2.run_example(0);
        assert_eq!(res, 123);
    }
}
