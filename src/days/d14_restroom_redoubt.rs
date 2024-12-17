use itertools::Itertools;

use crate::{
    day,
    utils::{Day, Task, read_lines},
};

type Point = (i32, i32);

fn parse_file(filename: &str) -> ((i32, i32, i32), impl Iterator<Item = (Point, Point)>) {
    let mut lines = read_lines(filename);

    let whs =
        lines.next().unwrap().split_whitespace().flat_map(str::parse).collect_tuple().unwrap();

    fn parse_pair(s: &str) -> Option<Point> {
        let (x, y) = s[2..].split_once(',')?;
        Some((x.parse().ok()?, y.parse().ok()?))
    }

    let iter = lines.flat_map(|line| {
        let (p, v) = line.split_once(' ')?;
        Some((parse_pair(p)?, parse_pair(v)?))
    });

    (whs, iter)
}

fn p1(filename: &str) -> usize {
    let ((w, h, steps), iter) = parse_file(filename);

    let x_mid = (w % 2 == 1).then_some(w / 2);
    let y_mid = (h % 2 == 1).then_some(h / 2);

    iter.flat_map(|((px, py), (vx, vy))| {
        let x = (vx * steps + px).rem_euclid(w);
        let y = (vy * steps + py).rem_euclid(h);
        (Some(x) != x_mid && Some(y) != y_mid).then_some((x, y))
    })
    .fold([0, 0, 0, 0], |mut arr, (x, y)| {
        arr[(((x + w / 2) / w) + 2 * ((y + h / 2) / h)) as usize] += 1;
        arr
    })
    .iter()
    .product()
}
fn p2(filename: &str) -> usize {
    let ((w, h, _), iter) = parse_file(filename);
    let robots = iter.collect_vec();

    let (step, points) = (0..)
        .find_map(|step| {
            let points = robots
                .iter()
                .map(|((px, py), (vx, vy))| {
                    let x = (vx * step + px).rem_euclid(w) as usize;
                    let y = (vy * step + py).rem_euclid(h) as usize;
                    (x, y)
                })
                .counts();
            points.iter().all(|(_, count)| *count == 1).then_some((step, points))
        })
        .unwrap();

    (0..h as usize).for_each(|y| {
        (0..w as usize).for_each(|x| match points.get(&(x, y)) {
            None => print!("."),
            Some(&c) => print!("{c}"),
        });
        println!();
    });

    step as _
}

pub const SOLUTION: Day<usize, usize> = day! { 14,
    part_1: { examples: ["example.txt"], func: p1 },
    part_2: { examples: ["example.txt"], func: p2 }
};

#[cfg(test)]
mod d14_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        assert_eq!(SOLUTION.part_1.run_example(0), 12);
    }

    #[test]
    fn playground() {
        SOLUTION.part_2.run_example(0);
    }
}
