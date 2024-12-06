use std::{collections::HashSet, ops::ControlFlow};

use itertools::Itertools;

use crate::utils::{Day, Task, read_lines};

#[rustfmt::skip]
#[derive(Copy, Clone)]
enum Tile { Free, Obstacle }

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
                Tile::Free => Some(Self::new(x, y - 1, dir)),
                Tile::Obstacle => Some(Self::new(x, y, R)),
            },
            R if x + 1 == w => None,
            R => match tiles[y][x + 1] {
                Tile::Free => Some(Self::new(x + 1, y, dir)),
                Tile::Obstacle => Some(Self::new(x, y, D)),
            },
            D if y + 1 == h => None,
            D => match tiles[y + 1][x] {
                Tile::Free => Some(Self::new(x, y + 1, dir)),
                Tile::Obstacle => Some(Self::new(x, y, L)),
            },
            L if x == 0 => None,
            L => match tiles[y][x - 1] {
                Tile::Free => Some(Self::new(x - 1, y, dir)),
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
                    '.' => Tile::Free,
                    '^' | '>' | 'v' | '<' => {
                        guard = Guard::new(x, y, Direction::from_char(c));
                        Tile::Free
                    }
                    '#' => Tile::Obstacle,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    (tiles, guard)
}

fn p1_path_length(filename: &str) -> usize {
    let (tiles, guard) = parse_input(filename);
    std::iter::successors(Some(guard), |g| g.advance(&tiles)).map(|g| (g.x, g.y)).unique().count()
}

fn p2_add_obstacle(filename: &str) -> usize {
    let (mut tiles, guard) = parse_input(filename);
    let start = guard;

    let mut path: HashSet<_> =
        std::iter::successors(Some(guard), |g| g.advance(&tiles)).map(|g| (g.x, g.y)).collect();

    path.remove(&(start.x, start.y));

    path.iter()
        .filter(|&&(x, y)| {
            tiles[y][x] = Tile::Obstacle;

            let stuck = std::iter::successors(Some(start), |g| g.advance(&tiles))
                .try_fold(HashSet::new(), |mut visited, g| match visited.insert(g) {
                    true => ControlFlow::Continue(visited),
                    false => ControlFlow::Break(()),
                })
                .is_break();

            tiles[y][x] = Tile::Free;
            stuck
        })
        .count()
}

pub const SOLUTION: Day<usize, usize> = Day {
    part_1: Task {
        examples: &["./inputs/day_06/example.txt"],
        task: "./inputs/day_06/task.txt",
        func: p1_path_length,
    },
    part_2: Task {
        examples: &["./inputs/day_06/example.txt"],
        task: "./inputs/day_06/task.txt",
        func: p2_add_obstacle,
    },
};

#[cfg(test)]
mod d06_tests {
    use crate::days::d06_guard_gallivant::SOLUTION;

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
