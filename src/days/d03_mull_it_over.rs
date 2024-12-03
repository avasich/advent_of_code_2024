use crate::utils::{Day, Task, read_lines};

fn p1_sum_of_mul(filename: &str) -> u32 {
    read_lines(filename)
        .map(|line| {
            std::iter::successors(Some((line.as_str(), None)), |&(suffix, _)| {
                let l_br = suffix.find("mul(")? + 3;
                let r_br_max = (l_br + 8).min(suffix.len() - 1);
                let substr = suffix[l_br + 1..=r_br_max]
                    .find(')')
                    .map(|r_br| &suffix[l_br + 1..=l_br + r_br]);
                (l_br + 5 < suffix.len()).then_some((&suffix[l_br + 1..], substr))
            })
            .filter_map(|(_, substr)| {
                let (a, b) = substr?.split_once(',')?;
                let a = a.parse::<u32>().ok()?;
                let b = b.parse::<u32>().ok()?;
                Some(a * b)
            })
            .sum::<u32>()
        })
        .sum()
}

pub const SOLUTION: Day<u32, u32> = Day {
    part_1: Task {
        examples: &["./inputs/day_03/example.txt"],
        task: "./inputs/day_03/task.txt",
        func: p1_sum_of_mul,
    },
    part_2: Task {
        examples: &["./inputs/day_03/example.txt"],
        task: "./inputs/day_03/task.txt",
        func: p1_sum_of_mul,
    },
};

#[cfg(test)]
mod d03_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        let res = SOLUTION.part_1.run_example(0);
        assert_eq!(res, 161);
    }

    #[test]
    fn foo() {
        let x = &"ab"[1..5];
        println!("{x:?}");
    }
}
