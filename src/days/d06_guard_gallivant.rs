use std::collections::HashSet;

use crate::utils::{Day, Task, read_lines};

#[derive(Copy, Clone)]
enum Tile {
    Free,
    Obstacle,
}

#[derive(Copy, Clone)]
enum Direction {
    U,
    R,
    D,
    L,
}

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

    // fn turn_right(&self) -> Self {
    //     match self {
    //         Self::U => Self::R,
    //         Self::R => Self::D,
    //         Self::D => Self::L,
    //         Self::L => Self::U,
    //     }
    // }
}

fn p1_path_length(filename: &str) -> usize {
    let (mut x, mut y) = (0, 0);
    let mut dir = Direction::U;

    let tiles: Vec<Vec<_>> = read_lines(filename)
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '.' => Tile::Free,
                    '^' | '>' | 'v' | '<' => {
                        (x, y, dir) = (col, row, Direction::from_char(c));
                        Tile::Free
                    }
                    '#' => Tile::Obstacle,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let (w, h) = (tiles[0].len(), tiles.len());

    let mut visited = HashSet::new();
    loop {
        use Direction::*;
        visited.insert((x, y));

        match dir {
            U if y == 0 => break,
            U => match tiles[y - 1][x] {
                Tile::Free => y -= 1,
                Tile::Obstacle => dir = R,
            },
            R if x + 1 == w => break,
            R => match tiles[y][x + 1] {
                Tile::Free => x += 1,
                Tile::Obstacle => dir = D,
            },
            D if y + 1 == h => break,
            D => match tiles[y + 1][x] {
                Tile::Free => y += 1,
                Tile::Obstacle => dir = L,
            },
            L if x == 0 => break,
            L => match tiles[y][x - 1] {
                Tile::Free => x -= 1,
                Tile::Obstacle => dir = U,
            },
        }
    }
    visited.len()
}

fn p2(filename: &str) -> usize {
    0
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
        func: p2,
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
        let _res = SOLUTION.part_1.run_example(0);
    }
}
