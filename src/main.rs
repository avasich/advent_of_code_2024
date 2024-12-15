fn main() {
    run_day(12);
}

fn run_day(day_number: usize) {
    use advent_of_code_2024::{days::*, utils::Solution};

    #[allow(clippy::zero_prefixed_literal)]
    let solution = match day_number {
        01 => d01_historian_hysteria::SOLUTION.boxed(),
        02 => d02_red_nosed_reports::SOLUTION.boxed(),
        03 => d03_mull_it_over::SOLUTION.boxed(),
        04 => d04_ceres_search::SOLUTION.boxed(),
        05 => d05_print_queue::SOLUTION.boxed(),
        06 => d06_guard_gallivant::SOLUTION.boxed(),
        07 => d07_bridge_repair::SOLUTION.boxed(),
        08 => d08_resonant_collinearity::SOLUTION.boxed(),
        09 => d09_disk_fragmenter::SOLUTION.boxed(),
        10 => d10_hoof_it::SOLUTION.boxed(),
        11 => d11_plutonian_pebbles::SOLUTION.boxed(),
        12 => d12_garden_groups::SOLUTION.boxed(),
        _ => unreachable!(),
    };

    solution.run_part_1();
    solution.run_part_2();
}
