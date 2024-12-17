use itertools::Itertools;

use crate::{
    day,
    utils::{Day, Task, read_lines},
};

#[derive(Copy, Clone, Debug)]
enum Tile {
    Wall,
    Box,
    Empty,
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    L,
    U,
    R,
    D,
}

type Point = (usize, usize);

impl Direction {
    fn update_pos(&self, (x, y): Point) -> Point {
        match self {
            Direction::L => (x - 1, y),
            Direction::U => (x, y - 1),
            Direction::R => (x + 1, y),
            Direction::D => (x, y + 1),
        }
    }
}

fn parse_map(lines: &mut impl Iterator<Item = String>) -> (Point, Vec<Vec<Tile>>) {
    let mut start = (0, 0);
    let map = std::iter::from_fn(|| lines.next().filter(|line| !line.is_empty()))
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => Tile::Empty,
                    'O' => Tile::Box,
                    '#' => Tile::Wall,
                    '@' => {
                        start = (x, y);
                        Tile::Empty
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    (start, map)
}

fn parse_moves(lines: &mut impl Iterator<Item = String>) -> impl Iterator<Item = Direction> {
    lines.flat_map(|line| {
        line.chars()
            .map(|c| match c {
                '<' => Direction::L,
                '^' => Direction::U,
                '>' => Direction::R,
                'v' => Direction::D,
                _ => unreachable!(),
            })
            .collect_vec()
    })
}

fn p1(filename: &str) -> usize {
    let mut lines = read_lines(filename);
    let (mut xy, mut map) = parse_map(&mut lines);

    fn go(start: Point, dir: Direction, map: &[Vec<Tile>]) -> Option<(Point, Option<Point>)> {
        let first_step = dir.update_pos(start);
        let (mut x, mut y) = first_step;

        loop {
            match map[y][x] {
                Tile::Wall => return None,
                Tile::Box => (x, y) = dir.update_pos((x, y)),
                Tile::Empty => {
                    return match (x, y) == first_step {
                        true => Some((first_step, None)),
                        false => Some((first_step, Some((x, y)))),
                    };
                }
            };
        }
    }

    parse_moves(&mut lines).for_each(|m| match go(xy, m, &map) {
        None => {}
        Some(((new_x, new_y), move_box)) => {
            xy = (new_x, new_y);
            if let Some((box_x, box_y)) = move_box {
                map[box_y][box_x] = Tile::Box;
                map[new_y][new_x] = Tile::Empty;
            }
        }
    });

    map.iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &t)| (x, y, t)))
        .filter_map(|(x, y, t)| matches!(t, Tile::Box).then_some(100 * y + x))
        .sum()
}
fn p2(filename: &str) -> usize {
    0
}

pub const SOLUTION: Day<usize, usize> = day! { 15,
    part_1: { examples: ["example_1.txt", "example_2.txt"], func: p1 },
    part_2: { examples: ["example_1.txt"], func: p2 }
};

#[cfg(test)]
mod d15_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        assert_eq!(SOLUTION.part_1.run_example(0), 10092);
        assert_eq!(SOLUTION.part_1.run_example(1), 2028);
    }

    #[test]
    fn p2_example_test() {
        assert_eq!(SOLUTION.part_2.run_example(0), 9021);
    }

    #[test]
    fn playground() {
        SOLUTION.part_1.run_example(1);
    }
}
