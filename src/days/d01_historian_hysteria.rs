use std::collections::HashMap;

use crate::{
    day,
    utils::{Day, Task, read_lines},
};

fn parse_file(filename: &str) -> (Vec<u32>, Vec<u32>) {
    read_lines(filename)
        .flat_map(|line| {
            let mut a = line.split_whitespace().flat_map(str::parse::<u32>);
            Some((a.next()?, a.next()?))
        })
        .unzip()
}

fn p1_list_distance(filename: &str) -> u32 {
    let (mut fst, mut snd) = parse_file(filename);
    fst.sort();
    snd.sort();
    fst.into_iter().zip(snd).map(|(a, b)| a.abs_diff(b)).sum()
}

fn p2_similarity_score(filename: &str) -> u32 {
    let (fst, snd) = parse_file(filename);
    let snd_counts = snd.iter().fold(HashMap::new(), |mut map, n| {
        *map.entry(n).or_insert(0) += 1;
        map
    });
    fst.iter().flat_map(|x| snd_counts.get(x).map(|y| x * y)).sum()
}

pub const SOLUTION: Day<u32, u32> = day! { 1,
    part_1: { examples: ["example.txt"], func: p1_list_distance },
    part_2: { examples: ["example.txt"], func: p2_similarity_score }
};

#[cfg(test)]
mod d01_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        let res = SOLUTION.part_1.run_example(0);
        assert_eq!(res, 11);
    }

    #[test]
    fn p2_example_test() {
        let res = SOLUTION.part_2.run_example(0);
        assert_eq!(res, 31);
    }
}
