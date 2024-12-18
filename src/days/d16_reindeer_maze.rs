use crate::{
    day,
    utils::{Day, Task},
};
fn p1(filename: &str) -> usize {
    0
}

fn p2(filename: &str) -> usize {
    0
}
pub const SOLUTION: Day<usize, usize> = day! { 16,
    part_1: { examples: ["example_1.txt", "example_2.txt"], func: p1 },
    part_2: { examples: ["example_1.txt"], func: p2 }
};

#[cfg(test)]
mod d16_tests {
    use super::*;
    
    #[test]
    fn p1_example_tests() {
        assert_eq!(SOLUTION.part_1.run_example(0), 0);
    }
}