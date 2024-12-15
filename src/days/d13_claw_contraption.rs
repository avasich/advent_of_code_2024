use crate::{
    day,
    utils::{Day, Task, read_lines},
};

type Point = (u64, u64);

fn parse_file(filename: &str) -> impl Iterator<Item = (Point, Point, Point)> {
    fn parse_xy(s: &str) -> Option<Point> {
        let (_, xy) = s.split_once(':')?;
        let (x, y) = xy.split_once(',')?;
        let (x, y) = (x.trim(), y.trim());
        Some((x[2..].parse().ok()?, y[2..].parse().ok()?))
    }

    read_lines(filename)
        .filter(|s| !s.is_empty())
        .array_chunks()
        .flat_map(|[a, b, p]| Some((parse_xy(&a)?, parse_xy(&b)?, parse_xy(&p)?)))
}

fn p1(filename: &str) -> u64 {
    parse_file(filename)
        .flat_map(|((ax, ay), (bx, by), (px, py))| {
            (0..)
                .map(move |i| (i, ax * i, ay * i))
                .take_while(move |&(_, x, y)| x <= px && y <= py)
                .filter_map(move |(i, x, y)| {
                    let (div_x, rem_x) = ((px - x) / bx, (px - x) % bx);
                    let (div_y, rem_y) = ((py - y) / by, (py - y) % by);
                    (rem_x == 0 && rem_y == 0 && div_x == div_y).then_some((i, div_x))
                })
                .map(|(an, bn)| 3 * an + bn)
                .min()
        })
        .sum()
}
fn p2(filename: &str) -> u64 {
    const ADD: u64 = 10000000000000;
    let _ = parse_file(filename).map(|(a, b, (px, py))| (a, b, (px + ADD, py + ADD)));
    0
}
pub const SOLUTION: Day<u64, u64> = day! { 13,
    part_1: { examples: ["example.txt"], func: p1 },
    part_2: { examples: ["example.txt"], func: p2 }
};

#[cfg(test)]
mod d13_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        assert_eq!(SOLUTION.part_1.run_example(0), 480);
    }

    #[test]
    fn p2_example_test() {
        assert_eq!(SOLUTION.part_2.run_example(0), 0);
    }
}
