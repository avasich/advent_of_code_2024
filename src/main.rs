fn main() {
    run_day(9);
}

fn run_day(day_number: usize) {
    use advent_of_code_2024::{days::*, utils::Solution};

    let solution: Box<dyn Solution> = match day_number {
        1 => Box::new(d01_historian_hysteria::SOLUTION),
        2 => Box::new(d02_red_nosed_reports::SOLUTION),
        3 => Box::new(d03_mull_it_over::SOLUTION),
        4 => Box::new(d04_ceres_search::SOLUTION),
        5 => Box::new(d05_print_queue::SOLUTION),
        6 => Box::new(d06_guard_gallivant::SOLUTION),
        7 => Box::new(d07_bridge_repair::SOLUTION),
        8 => Box::new(d08_resonant_collinearity::SOLUTION),
        9 => Box::new(d09_disk_fragmenter::SOLUTION),
        _ => unreachable!(),
    };

    solution.run_part_1();
    solution.run_part_2();
}
