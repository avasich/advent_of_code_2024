use crate::{
    day,
    utils::{Day, Task},
};

fn p1(filename: &str) -> u64 {
    0
}

fn p2(filename: &str) -> u64 {
    0
}

pub const SOLUTION: Day<u64, u64> = day! { 10,
    part_1: { examples: ["example_1.txt", "example_2.txt"], func: p1 },
    part_2: { examples: ["example_1.txt", "example_2.txt"], func: p2 }
};

#[cfg(test)]
mod d10_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        let res = SOLUTION.part_1.run_example(0);
        assert_eq!(res, 0);
    }

    #[test]
    fn p2_example_test() {
        let res = SOLUTION.part_2.run_example(0);
        assert_eq!(res, 0);
    }
}
