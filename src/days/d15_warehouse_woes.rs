use std::collections::HashSet;

use itertools::Itertools;

use crate::{
    day,
    utils::{Day, Task, read_lines},
};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Tile {
    Wall,
    BoxL,
    BoxR,
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
    fn update_pos(&self) -> fn(Point) -> Point {
        match self {
            Direction::L => |(x, y)| (x - 1, y),
            Direction::U => |(x, y)| (x, y - 1),
            Direction::R => |(x, y)| (x + 1, y),
            Direction::D => |(x, y)| (x, y + 1),
        }
    }
}

fn parse_map(lines: &mut impl Iterator<Item = String>, widen: bool) -> (Point, Vec<Vec<Tile>>) {
    let mut start = (0, 0);
    let map = std::iter::from_fn(|| lines.next().filter(|line| !line.is_empty()))
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .flat_map(|(x, c)| match (c, widen) {
                    ('.', false) => [Tile::Empty].as_slice().iter(),
                    ('O', false) => [Tile::BoxL].as_slice().iter(),
                    ('#', false) => [Tile::Wall].as_slice().iter(),
                    ('@', false) => {
                        start = (x, y);
                        [Tile::Empty].as_slice().iter()
                    }
                    ('.', true) => [Tile::Empty, Tile::Empty].as_slice().iter(),
                    ('O', true) => [Tile::BoxL, Tile::BoxR].as_slice().iter(),
                    ('#', true) => [Tile::Wall, Tile::Wall].as_slice().iter(),
                    ('@', true) => {
                        start = (2 * x, y);
                        [Tile::Empty, Tile::Empty].as_slice().iter()
                    }
                    _ => unreachable!(),
                })
                .copied()
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

fn weight_map(map: &[Vec<Tile>]) -> usize {
    map.iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &t)| (x, y, t)))
        .filter_map(|(x, y, t)| matches!(t, Tile::BoxL).then_some(100 * y + x))
        .sum()
}

fn p1(filename: &str) -> usize {
    let mut lines = read_lines(filename);
    let (mut xy, mut map) = parse_map(&mut lines, false);

    fn go(start: Point, dir: Direction, map: &[Vec<Tile>]) -> Option<(Point, Point)> {
        let update_pos = dir.update_pos();
        let first_step = update_pos(start);
        let (mut x, mut y) = first_step;

        loop {
            match map[y][x] {
                Tile::Wall => return None,
                Tile::BoxL => (x, y) = update_pos((x, y)),
                Tile::Empty => return Some((first_step, (x, y))),
                _ => unreachable!(),
            };
        }
    }

    parse_moves(&mut lines).for_each(|m| match go(xy, m, &map) {
        None => {}
        Some(((new_x, new_y), (free_x, free_y))) => {
            map[free_y][free_x] = map[new_y][new_x];
            map[new_y][new_x] = Tile::Empty;
            xy = (new_x, new_y);
        }
    });

    weight_map(&map)
}

fn p2(filename: &str) -> usize {
    let mut lines = read_lines(filename);
    let ((mut x, mut y), mut map) = parse_map(&mut lines, true);

    enum Move {
        None,
        Horizontal(usize, usize),
        Vertical(usize, Vec<(Point, Point, Tile)>),
    }

    fn go(start: Point, dir: Direction, map: &[Vec<Tile>]) -> Move {
        let update_pos = dir.update_pos();
        let first_step = update_pos(start);

        match dir {
            Direction::L | Direction::R => {
                let (mut x, mut y) = update_pos(start);
                loop {
                    match map[y][x] {
                        Tile::Wall => return Move::None,
                        Tile::BoxL | Tile::BoxR => (x, y) = update_pos((x, y)),
                        Tile::Empty => return Move::Horizontal(update_pos(start).0, x),
                    }
                }
            }
            Direction::U | Direction::D => {
                let mut front = HashSet::from([start]);
                let mut affected = vec![];

                loop {
                    let new_front = front.iter().copied().map(update_pos).try_fold(
                        HashSet::new(),
                        |mut fr, (x, y)| match map[y][x] {
                            Tile::Wall => None,
                            Tile::BoxL => {
                                fr.extend([(x, y), (x + 1, y)]);
                                Some(fr)
                            }
                            Tile::BoxR => {
                                fr.extend([(x - 1, y), (x, y)]);
                                Some(fr)
                            }
                            Tile::Empty => Some(fr),
                        },
                    );

                    match new_front {
                        None => return Move::None,
                        Some(f) if f.is_empty() => return Move::Vertical(first_step.1, affected),
                        Some(f) => {
                            affected.extend(f.iter().map(|&(box_x, box_y)| {
                                ((box_x, box_y), update_pos((box_x, box_y)), map[box_y][box_x])
                            }));
                            front = f;
                        }
                    }
                }
            }
        }
    }

    parse_moves(&mut lines).for_each(|m| match go((x, y), m, &map) {
        Move::None => {}
        Move::Horizontal(new_x, push_to) => {
            match x > push_to {
                true => (push_to..x).for_each(|bx| map[y][bx] = map[y][bx + 1]),
                false => (x + 1..push_to + 1).rev().for_each(|bx| map[y][bx] = map[y][bx - 1]),
            }
            x = new_x;
        }
        Move::Vertical(new_y, mut boxes) => {
            while let Some(((bx, by), (new_bx, new_by), t)) = boxes.pop() {
                map[by][bx] = Tile::Empty;
                map[new_by][new_bx] = t;
            }
            y = new_y;
        }
    });

    weight_map(&map)
}

#[allow(unused)]
fn print_map(map: &[Vec<Tile>], xy: (usize, usize), widen: bool) {
    map.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, t)| match t {
            _ if (x, y) == xy => print!("@"),
            Tile::Wall => print!("#"),
            Tile::BoxL if widen => print!("["),
            Tile::BoxL => println!("."),
            Tile::BoxR => print!("]"),
            Tile::Empty => print!("."),
        });
        println!();
    });
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
        SOLUTION.part_2.run_example(0);
    }
}
