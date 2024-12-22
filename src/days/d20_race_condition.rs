use std::collections::{HashMap, HashSet, VecDeque};

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

fn p1(filename: &str) -> usize {
    let (start, end, map) = parse_map(filename);

    let delta = 100;
    let distances = calculate_distances(end, &map);
    let t0 = *distances.get(&start).unwrap();

    let (w, h) = (map[0].len(), map.len());

    let mut q = VecDeque::from([(start, 0, None::<[(usize, usize); 2]>)]);
    let mut visited = HashSet::new();

    let mut cheats = HashMap::new();

    while let Some(pos) = q.pop_front() {
        let ((x0, y0), steps, cheat) = pos;

        if !visited.insert(((x0, y0), cheat)) {
            continue;
        }

        match (distances.get(&(x0, y0)), cheat) {
            (None, None) => moves(x0, y0, w, h)
                .into_iter()
                .flatten()
                .for_each(|(x, y)| q.push_back(((x, y), steps + 1, Some([(x0, y0), (x, y)])))),
            (None, _) => {}
            (Some(_), None) => moves(x0, y0, w, h)
                .into_iter()
                .flatten()
                .for_each(|(x, y)| q.push_back(((x, y), steps + 1, cheat))),
            (Some(&d), Some(cheat)) => {
                cheats.insert(cheat, steps + d);
            }
        }
    }

    cheats.iter().filter(|&(_, &t)| t + delta <= t0).count()
}

fn moves(x: usize, y: usize, w: usize, h: usize) -> [Option<(usize, usize)>; 4] {
    [
        (x > 1).then(|| (x - 1, y)),
        (y > 1).then(|| (x, y - 1)),
        (x + 2 < w).then_some((x + 1, y)),
        (y + 2 < h).then_some((x, y + 1)),
    ]
}

fn calculate_distances((x_end, y_end): Point, map: &[Vec<Tile>]) -> HashMap<(usize, usize), i32> {
    let (w, h) = (map[0].len(), map.len());
    let mut q = VecDeque::from([(x_end, y_end, 0)]);
    let mut distances = HashMap::from([((x_end, y_end), 0)]);

    while let Some((x, y, steps)) = q.pop_front() {
        moves(x, y, w, h).into_iter().flatten().for_each(|(x, y)| match map[y][x] {
            Tile::Empty if !distances.contains_key(&(x, y)) => {
                distances.insert((x, y), steps + 1);
                q.push_back((x, y, steps + 1));
            }
            _ => {}
        });
    }

    distances
}

fn p2(filename: &str) -> usize {
    0
}

pub const SOLUTION: Day<usize, usize> = day! { 20,
    part_1: { examples: ["example.txt"], func: p1 },
    part_2: { examples: ["example.txt"], func: p2 }
};

#[cfg(test)]
mod d20_tests {
    use super::*;

    #[test]
    fn p1_examples_test() {
        // 1310 too low
        // 1327 too low
        // assert_eq!(SOLUTION.part_1.run_example(0), 0);
    }

    #[test]
    fn p2_examples_test() {
        assert_eq!(SOLUTION.part_2.run_example(0), 0);
    }

    #[test]
    fn playground() {
        let res = SOLUTION.part_1.run_example(0);
        println!("{res}");
    }
}

/*







0123456789ABCDE
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############


 */
