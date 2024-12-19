use advent_of_code_2024::{days::*, utils::Solution};

fn main() {
    run_day(17);
}

fn run_day(day_number: usize) {
    #[allow(clippy::zero_prefixed_literal)]
    let solution: &dyn Solution = match day_number {
        01 => &d01_historian_hysteria::SOLUTION,
        02 => &d02_red_nosed_reports::SOLUTION,
        03 => &d03_mull_it_over::SOLUTION,
        04 => &d04_ceres_search::SOLUTION,
        05 => &d05_print_queue::SOLUTION,
        06 => &d06_guard_gallivant::SOLUTION,
        07 => &d07_bridge_repair::SOLUTION,
        08 => &d08_resonant_collinearity::SOLUTION,
        09 => &d09_disk_fragmenter::SOLUTION,
        10 => &d10_hoof_it::SOLUTION,
        11 => &d11_plutonian_pebbles::SOLUTION,
        12 => &d12_garden_groups::SOLUTION,
        13 => &d13_claw_contraption::SOLUTION,
        14 => &d14_restroom_redoubt::SOLUTION,
        15 => &d15_warehouse_woes::SOLUTION,
        16 => &d16_reindeer_maze::SOLUTION,
        17 => &d17_chronospatial_computer::SOLUTION,
        _ => unreachable!(),
    };

    solution.run_part_1();
    solution.run_part_2();
}
