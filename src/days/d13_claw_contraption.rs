use crate::{
    day,
    utils::{Day, Task, read_lines},
};

type Point = (i64, i64);

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

fn intersect((ax, ay): Point, (bx, by): Point, (px, py): Point) -> Option<Point> {
    use num::integer::div_rem;
    // i = (by * px - bx * py) / (ax * by - ay * bx)
    // j = (ax * py - ay * px) / (ax * by - ay * bx)

    match ax * by - ay * bx {
        0 => None,
        det => {
            let i = match div_rem(by * px - bx * py, det) {
                (i @ 0.., 0) => i,
                _ => return None,
            };
            let j = match div_rem(ax * py - ay * px, det) {
                (j @ 0.., 0) => j,
                _ => return None,
            };
            Some((i, j))
        }
    }
}

fn price(filename: &str, add: i64) -> i64 {
    parse_file(filename)
        .flat_map(|(a, b, (px, py))| intersect(a, b, (px + add, py + add)))
        .map(|(i, j)| 3 * i + j)
        .sum()
}

fn p1(filename: &str) -> i64 {
    price(filename, 0)
}

fn p2(filename: &str) -> i64 {
    price(filename, 10000000000000)
}

pub const SOLUTION: Day<i64, i64> = day! { 13,
    part_1: { examples: ["example.txt"], func: p1 },
    part_2: { examples: [], func: p2 }
};

#[cfg(test)]
mod d13_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        assert_eq!(SOLUTION.part_1.run_example(0), 480);
    }
}
