use std::collections::HashSet;

use itertools::Itertools;
use crate::day;
use crate::utils::{Day, Task, read_lines};

#[rustfmt::skip]
#[derive(Copy, Clone)]
enum Tile { Empty, Obstacle }

#[rustfmt::skip]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Direction { U, R, D, L }

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            '^' => Self::U,
            '>' => Self::R,
            'v' => Self::D,
            '<' => Self::L,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Guard {
    x: usize,
    y: usize,
    dir: Direction,
}

impl Guard {
    fn new(x: usize, y: usize, dir: Direction) -> Self {
        Self { x, y, dir }
    }

    fn advance(self, tiles: &[Vec<Tile>]) -> Option<Self> {
        use Direction::*;
        let (w, h) = (tiles[0].len(), tiles.len());
        let Self { x, y, dir } = self;

        match self.dir {
            U if self.y == 0 => None,
            U => match tiles[y - 1][x] {
                Tile::Empty => Some(Self::new(x, y - 1, dir)),
                Tile::Obstacle => Some(Self::new(x, y, R)),
            },
            R if x + 1 == w => None,
            R => match tiles[y][x + 1] {
                Tile::Empty => Some(Self::new(x + 1, y, dir)),
                Tile::Obstacle => Some(Self::new(x, y, D)),
            },
            D if y + 1 == h => None,
            D => match tiles[y + 1][x] {
                Tile::Empty => Some(Self::new(x, y + 1, dir)),
                Tile::Obstacle => Some(Self::new(x, y, L)),
            },
            L if x == 0 => None,
            L => match tiles[y][x - 1] {
                Tile::Empty => Some(Self::new(x - 1, y, dir)),
                Tile::Obstacle => Some(Self::new(x, y, U)),
            },
        }
    }
}

fn parse_input(filename: &str) -> (Vec<Vec<Tile>>, Guard) {
    let mut guard = Guard::new(0, 0, Direction::U);

    let tiles = read_lines(filename)
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => Tile::Empty,
                    '^' | '>' | 'v' | '<' => {
                        guard = Guard::new(x, y, Direction::from_char(c));
                        Tile::Empty
                    }
                    '#' => Tile::Obstacle,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    (tiles, guard)
}

fn p1(filename: &str) -> usize {
    let (tiles, guard) = parse_input(filename);
    p1_path_length(tiles, guard)
}

fn p1_path_length(tiles: Vec<Vec<Tile>>, guard: Guard) -> usize {
    std::iter::successors(Some(guard), |g| g.advance(&tiles)).map(|g| (g.x, g.y)).unique().count()
}

fn p2(filename: &str) -> usize {
    let (mut tiles, guard) = parse_input(filename);
    p2_add_obstacle(&mut tiles, guard)
}

fn p2_add_obstacle(tiles: &mut [Vec<Tile>], start: Guard) -> usize {
    let path: Vec<_> = std::iter::successors(Some(start), |g| g.advance(tiles)).collect();

    path.iter()
        .tuple_windows::<(_, _)>()
        .scan(HashSet::<(usize, usize)>::new(), |visited, (&g1, &g2)| {
            match visited.insert((g2.x, g2.y)) && g2 != start {
                true => Some(Some((g1, g2))),
                false => Some(None),
            }
        })
        .flatten()
        .filter(|&(g1, g2)| {
            tiles[g2.y][g2.x] = Tile::Obstacle;
            let stuck = std::iter::successors(Some(g1), |g| g.advance(tiles))
                .try_fold(HashSet::new(), |mut loop_visited, g| match loop_visited.insert(g) {
                    true => Some(loop_visited),
                    false => None,
                })
                .is_none();
            tiles[g2.y][g2.x] = Tile::Empty;
            stuck
        })
        .count()
}

pub const SOLUTION: Day<usize, usize> = day! { 6,
    part_1: { examples: ["example.txt"], func: p1 },
    part_2: { examples: ["example.txt"], func: p2 }
};

#[cfg(test)]
mod d06_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        let res = SOLUTION.part_1.run_example(0);
        assert_eq!(res, 41);
    }

    #[test]
    fn p2_example_test() {
        let res = SOLUTION.part_2.run_example(0);
        assert_eq!(res, 6);
    }
}
