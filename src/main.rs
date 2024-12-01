fn main() {
    run_day(1);
}

fn run_day(day_number: usize) {
    use advent_of_code_2024::{days::*, utils::Solution};

    let solution: Box<&dyn Solution> = match day_number {
        1 => Box::new(&d01_historian_hysteria::DAY),
        _ => unreachable!(),
    };

    solution.run_part_1();
    solution.run_part_2();
}
