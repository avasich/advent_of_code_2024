use crate::utils::{Day, Task};

fn p1(filename: &str) -> u64 {
    0
}

fn p2(filename: &str) -> u64 {
    0
}

pub const SOLUTION: Day<u64, u64> = Day {
    day: 10,
    part_1: Task { examples: &["example.txt"], func: p1 },
    part_2: Task { examples: &["example.txt"], func: p2 },
};

#[cfg(test)]
mod d10_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        let res = SOLUTION.run_example_1(0);
        assert_eq!(res, 3749);
    }

    #[test]
    fn p2_example_test() {
        let res = SOLUTION.run_example_2(0);
        assert_eq!(res, 11387);
    }
}
