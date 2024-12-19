use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use crate::{
    day,
    utils::{Day, Task, read_lines},
};

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
}

type Point = (usize, usize);

fn parse_map(filename: &str) -> (Point, Point, Vec<Vec<Tile>>) {
    let (mut start, mut end) = ((0, 0), (0, 0));

    let map = read_lines(filename)
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Empty,
                    'S' => {
                        start = (x, y);
                        Tile::Empty
                    }
                    'E' => {
                        end = (x, y);
                        Tile::Empty
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    (start, end, map)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    W,
    N,
    E,
    S,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Position {
    cost: i32,
    xy: Point,
    dir: Direction,
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Position {
    fn new(cost: i32, xy: Point, dir: Direction) -> Self {
        Self { cost, xy, dir }
    }

    fn moves(&self) -> [Self; 3] {
        let Self { cost, xy: (x, y), dir } = *self;

        match dir {
            Direction::W => [
                Self::new(cost + 1, (x - 1, y), Direction::W),
                Self::new(cost + 1000, (x, y), Direction::S),
                Self::new(cost + 1000, (x, y), Direction::N),
            ],
            Direction::N => [
                Self::new(cost + 1, (x, y - 1), Direction::N),
                Self::new(cost + 1000, (x, y), Direction::W),
                Self::new(cost + 1000, (x, y), Direction::E),
            ],
            Direction::E => [
                Self::new(cost + 1, (x + 1, y), Direction::E),
                Self::new(cost + 1000, (x, y), Direction::N),
                Self::new(cost + 1000, (x, y), Direction::S),
            ],
            Direction::S => [
                Self::new(cost + 1, (x, y + 1), Direction::S),
                Self::new(cost + 1000, (x, y), Direction::E),
                Self::new(cost + 1000, (x, y), Direction::W),
            ],
        }
    }
}

fn p1(filename: &str) -> i32 {
    let (start, end, map) = parse_map(filename);
    let start = Position::new(0, start, Direction::E);

    let mut q = BinaryHeap::from([start]);
    let mut visited = HashSet::new();

    while let Some(pos) = q.pop() {
        if !visited.insert((pos.xy, pos.dir)) {
            continue;
        }

        for p in pos.moves() {
            let (x, y) = p.xy;
            match map[y][x] {
                _ if (x, y) == end => return p.cost,
                Tile::Empty => q.push(p),
                Tile::Wall => continue,
            }
        }
    }
    0
}

fn p2(filename: &str) -> usize {
    let (start, end, map) = parse_map(filename);
    let start = Position::new(0, start, Direction::E);

    let mut q = BinaryHeap::from([start]);
    let mut visited = HashSet::new();
    let mut backtrack: HashMap<_, Vec<_>> = HashMap::new();

    let mut finishes: Vec<Position> = vec![];

    while let Some(pos) = q.pop() {
        if !visited.insert((pos.xy, pos.dir)) {
            continue;
        }

        if finishes.first().is_some_and(|&f| f.cost < pos.cost) {
            continue;
        }

        for p in pos.moves() {
            let (x, y) = p.xy;
            match map[y][x] {
                _ if p.xy == end => {
                    backtrack.entry(p).or_default().push(pos);
                    finishes.push(p);
                }
                Tile::Empty => {
                    backtrack.entry(p).or_default().push(pos);
                    q.push(p);
                }
                Tile::Wall => continue,
            }
        }
    }

    let mut tiles = HashSet::new();
    let mut q = finishes;

    while let Some(pos) = q.pop() {
        tiles.insert(pos.xy);
        if let Some(prev) = backtrack.get(&pos) {
            q.extend(prev);
        }
    }

    tiles.len()
}
pub const SOLUTION: Day<i32, usize> = day! { 16,
    part_1: { examples: ["example_1.txt", "example_2.txt"], func: p1 },
    part_2: { examples: ["example_1.txt", "example_2.txt"], func: p2 }
};

#[cfg(test)]
mod d16_tests {
    use super::*;

    #[test]
    fn p1_example_tests() {
        assert_eq!(SOLUTION.part_1.run_example(0), 7036);
        assert_eq!(SOLUTION.part_1.run_example(1), 11048);
    }

    #[test]
    fn p2_example_tests() {
        assert_eq!(SOLUTION.part_2.run_example(0), 45);
        assert_eq!(SOLUTION.part_2.run_example(1), 64);
    }

    #[test]
    fn playground() {
        SOLUTION.part_2.run_example(1);
    }
}
