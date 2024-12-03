use crate::utils::{Day, Task, read_lines};

fn try_mul(s: &str) -> Option<u32> {
    let (a, b) = s.split_once(',')?;
    let a = a.parse::<u32>().ok()?;
    let b = b.parse::<u32>().ok()?;
    Some(a * b)
}

fn p1_sum_of_mul(filename: &str) -> u32 {
    read_lines(filename)
        .map(|line| {
            std::iter::successors(Some((line.as_str(), None)), |&(suffix, _)| {
                let l_br = suffix.find("mul(")? + 3;
                let r_br_max = (l_br + 8).min(suffix.len() - 1);
                let substr = suffix[l_br + 1..=r_br_max]
                    .find(')')
                    .map(|r_br| &suffix[l_br + 1..=l_br + r_br]);
                Some((&suffix[l_br + 1..], substr))
            })
            .filter_map(|(_, substr)| try_mul(substr?))
            .sum::<u32>()
        })
        .sum()
}

fn p1_sum_of_mul_enable(filename: &str) -> u32 {
    read_lines(filename)
        .scan(true, |enabled, line| {
            let res = std::iter::successors(Some((line.as_str(), None)), |&(suffix, _)| {
                let l_br = suffix.find("mul(")? + 3;
                let enable = suffix[..l_br].rfind("do()");
                let disable = suffix[..l_br].rfind("don't()");
                *enabled = match (enable, disable) {
                    (Some(e), Some(d)) => e > d,
                    (Some(_), None) => true,
                    (None, Some(_)) => false,
                    _ => *enabled,
                };

                let substr = match enabled {
                    true => {
                        let r_br_max = (l_br + 8).min(suffix.len() - 1);
                        suffix[l_br + 1..=r_br_max]
                            .find(')')
                            .map(|r_br| &suffix[l_br + 1..=l_br + r_br])
                    }
                    false => None,
                };
                Some((&suffix[l_br + 1..], substr))
            })
            .filter_map(|(_, substr)| try_mul(substr?))
            .sum::<u32>();
            Some(res)
        })
        .sum()
}

pub const SOLUTION: Day<u32, u32> = Day {
    part_1: Task {
        examples: &["./inputs/day_03/example_1.txt"],
        task: "./inputs/day_03/task.txt",
        func: p1_sum_of_mul,
    },
    part_2: Task {
        examples: &["./inputs/day_03/example_2.txt"],
        task: "./inputs/day_03/task.txt",
        func: p1_sum_of_mul_enable,
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
    fn p2_example_test() {
        let res = SOLUTION.part_2.run_example(0);
        assert_eq!(res, 48);
    }

    #[test]
    fn foo() {
        let x = &"ab"[1..5];
        println!("{x:?}");
    }
}
