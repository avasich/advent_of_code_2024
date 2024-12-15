fn main() {
    run_day(11);
}

fn run_day(day_number: usize) {
    use advent_of_code_2024::{days::*, utils::Solution};

    #[allow(clippy::zero_prefixed_literal)]
    let solution: Box<dyn Solution> = match day_number {
        01 => Box::new(d01_historian_hysteria::SOLUTION),
        02 => Box::new(d02_red_nosed_reports::SOLUTION),
        03 => Box::new(d03_mull_it_over::SOLUTION),
        04 => Box::new(d04_ceres_search::SOLUTION),
        05 => Box::new(d05_print_queue::SOLUTION),
        06 => Box::new(d06_guard_gallivant::SOLUTION),
        07 => Box::new(d07_bridge_repair::SOLUTION),
        08 => Box::new(d08_resonant_collinearity::SOLUTION),
        09 => Box::new(d09_disk_fragmenter::SOLUTION),
        10 => Box::new(d10_hoof_it::SOLUTION),
        11 => Box::new(d11_plutonian_pebbles::SOLUTION),
        _ => unreachable!(),
    };

    solution.run_part_1();
    solution.run_part_2();
}
