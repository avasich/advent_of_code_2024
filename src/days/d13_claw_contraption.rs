use crate::{
    day,
    utils::{Day, Task, read_lines},
};

type Point = (i64, i64);

fn parse_file(filename: &str) -> impl Iterator<Item = [Point; 3]> {
    read_lines(filename)
        .flat_map(|s| {
            let (_, xy) = s.split_once(':')?;
            let (x, y) = xy.split_once(',')?;
            let (x, y) = (x.trim(), y.trim());
            Some((x[2..].parse().ok()?, y[2..].parse().ok()?))
        })
        .array_chunks()
}

fn intersect((ax, ay): Point, (bx, by): Point, (px, py): Point) -> Option<Point> {
    fn solve(v: i64, det: i64) -> Option<i64> {
        let (k, d) = num::integer::div_rem(v, det);
        (k >= 0 && d == 0).then_some(k)
    }

    match ax * by - ay * bx {
        0 => None,
        det => {
            let i = solve(by * px - bx * py, det)?;
            let j = solve(ax * py - ay * px, det)?;
            Some((i, j))
        }
    }
}

fn price(filename: &str, add: i64) -> i64 {
    parse_file(filename)
        .flat_map(|[a, b, (px, py)]| intersect(a, b, (px + add, py + add)))
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
