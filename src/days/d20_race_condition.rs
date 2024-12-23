use std::collections::{HashMap, HashSet, VecDeque};

use crate::{
    day,
    utils::{Day, Task, read_lines},
};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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

fn simulate(filename: &str, cheat_len: i32, threshold: i32) -> usize {
    let (start, end, map) = parse_map(filename);
    let (w, h) = (map[0].len(), map.len());

    let distances = find_distances(end, &map);

    let mut q = VecDeque::from([start]);
    let mut visited = HashSet::new();
    let mut cheats = HashMap::new();

    while let Some(xy0) = q.pop_front() {
        if !visited.insert(xy0) {
            continue;
        }

        match distances.get(&xy0) {
            None => {}
            Some(&d) => {
                q.extend(moves(xy0, w, h).into_iter().flatten());

                find_exits(xy0, &map, cheat_len).iter().for_each(|&(cheat_d, cheat_end)| {
                    let cheat_d = distances.get(&cheat_end).unwrap() + cheat_d;
                    cheats.insert((xy0, cheat_end), d - cheat_d);
                });
            }
        }
    }

    cheats.iter().filter(|&(_, &d)| d >= threshold).count()
}

fn moves((x, y): Point, w: usize, h: usize) -> [Option<(usize, usize)>; 4] {
    [
        (x > 0).then(|| (x - 1, y)),
        (y > 0).then(|| (x, y - 1)),
        (x + 1 < w).then_some((x + 1, y)),
        (y + 1 < h).then_some((x, y + 1)),
    ]
}

fn find_distances(end: Point, map: &[Vec<Tile>]) -> HashMap<Point, i32> {
    let (w, h) = (map[0].len(), map.len());
    let mut q = VecDeque::from([(end, 0)]);
    let mut distances = HashMap::from([(end, 0)]);

    while let Some((xy, steps)) = q.pop_front() {
        moves(xy, w, h).into_iter().flatten().for_each(|(x, y)| match map[y][x] {
            Tile::Empty if !distances.contains_key(&(x, y)) => {
                distances.insert((x, y), steps + 1);
                q.push_back(((x, y), steps + 1));
            }
            _ => {}
        });
    }

    distances
}

fn find_exits((x0, y0): Point, map: &[Vec<Tile>], cheat_len: i32) -> Vec<(i32, Point)> {
    let (w, h) = (map[0].len(), map.len());

    let up = y0.saturating_sub(cheat_len as usize);
    let down = (y0 + cheat_len as usize).min(h - 1);

    (up..=down)
        .flat_map(|y| {
            let dx = cheat_len as usize - y0.abs_diff(y);
            let left = x0.saturating_sub(dx);
            let right = (x0 + dx).min(w - 1);
            (left..=right).map(move |x| (x, y))
        })
        .filter(|&(x, y)| matches!(map[y][x], Tile::Empty))
        .map(|(x, y)| ((x0.abs_diff(x) + y0.abs_diff(y)) as i32, (x, y)))
        .collect()
}

fn p1(filename: &str) -> usize {
    simulate(filename, 2, 100)
}

fn p2(filename: &str) -> usize {
    simulate(filename, 20, 100)
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
...x...
..xxx..
.xxoxx.
..xxx..
...x...



(((8, 7), (1, 9)), 50)
(((9, 7), (2, 9)), 50)
(((5, 2), (4, 11)), 50)
(((1, 3), (6, 13)), 50)
(((5, 2), (3, 11)), 50)
(((5, 2), (5, 11)), 50)
(((9, 3), (4, 7)), 50)
(((9, 2), (5, 7)), 50)
(((5, 2), (2, 13)), 50)
(((9, 4), (3, 7)), 50)

(((3, 2), (5, 11)), 52)
(((8, 7), (2, 9)), 52)
(((9, 3), (5, 7)), 52)
(((5, 3), (2, 13)), 52)
(((7, 2), (1, 9)), 52)
(((5, 3), (5, 11)), 52)
(((2, 1), (5, 11)), 52)
(((7, 5), (1, 9)), 52)
(((7, 1), (1, 9)), 52)
(((5, 3), (4, 11)), 52)
(((5, 3), (3, 11)), 52)
(((9, 5), (3, 7)), 52)
(((4, 3), (5, 11)), 52)
(((7, 4), (1, 9)), 52)
(((7, 3), (1, 9)), 52)
(((7, 6), (1, 9)), 52)
(((9, 4), (4, 7)), 52)
(((7, 7), (1, 9)), 52)
(((3, 1), (5, 11)), 52)
(((3, 3), (5, 11)), 52)

(((3, 3), (4, 11)), 54)
(((9, 4), (5, 7)), 54)
(((7, 1), (2, 9)), 54)
(((9, 5), (4, 7)), 54)
(((9, 6), (3, 7)), 54)
(((7, 5), (2, 9)), 54)
(((1, 2), (5, 11)), 54)
(((7, 3), (2, 9)), 54)
(((2, 1), (4, 11)), 54)
(((7, 7), (2, 9)), 54)
(((4, 3), (2, 13)), 54)
(((4, 3), (4, 11)), 54)
(((4, 3), (3, 11)), 54)
(((3, 2), (4, 11)), 54)
(((7, 6), (2, 9)), 54)
(((6, 1), (1, 9)), 54)
(((3, 1), (4, 11)), 54)
(((7, 2), (2, 9)), 54)
(((7, 4), (2, 9)), 54)

(((9, 5), (5, 7)), 56)
(((3, 1), (3, 11)), 56)
(((6, 1), (2, 9)), 56)
(((3, 3), (3, 11)), 56)
(((5, 1), (1, 9)), 56)
(((3, 1), (2, 13)), 56)
(((2, 1), (3, 11)), 56)
(((3, 3), (2, 13)), 56)
(((3, 2), (3, 11)), 56)
(((1, 3), (5, 11)), 56)
(((9, 6), (4, 7)), 56)
(((3, 2), (2, 13)), 56)
(((1, 2), (4, 11)), 56)

(((1, 3), (4, 11)), 58)
(((5, 2), (1, 9)), 58)
(((5, 1), (2, 9)), 58)
(((9, 6), (5, 7)), 58)
(((2, 1), (2, 13)), 58)
(((1, 2), (3, 11)), 58)

11 23
(((5, 2), (2, 9)), 60)
(((7, 1), (3, 7)), 60)
(((7, 3), (3, 7)), 60)
(((5, 3), (1, 9)), 60)
(((9, 7), (5, 7)), 60)
(((1, 3), (3, 11)), 60)
(((7, 4), (3, 7)), 60)
(((7, 2), (3, 7)), 60)
(((7, 6), (3, 7)), 60)
(((1, 2), (2, 13)), 60)
(((7, 5), (3, 7)), 60)

11 20
(((7, 2), (4, 7)), 62)
(((7, 3), (4, 7)), 62)
(((1, 3), (2, 13)), 62)
(((6, 1), (3, 7)), 62)
(((7, 1), (4, 7)), 62)
(((5, 3), (2, 9)), 62)
(((4, 3), (1, 9)), 62)
(((7, 4), (4, 7)), 62)
(((7, 5), (4, 7)), 62)
(((8, 7), (5, 7)), 62)
(((7, 6), (4, 7)), 62)

13 19
(((4, 3), (2, 9)), 64)
(((7, 2), (5, 7)), 64)
(((7, 3), (5, 7)), 64)
(((7, 5), (5, 7)), 64)
(((7, 6), (5, 7)), 64)
(((3, 3), (1, 9)), 64)
(((7, 7), (5, 7)), 64)
(((7, 1), (5, 7)), 64)
(((3, 2), (1, 9)), 64)
(((3, 1), (1, 9)), 64)
(((5, 1), (3, 7)), 64)
(((6, 1), (4, 7)), 64)
(((7, 4), (5, 7)), 64)

7 12
(((5, 1), (4, 7)), 66)
(((2, 1), (1, 9)), 66)
(((3, 1), (2, 9)), 66)
(((5, 2), (3, 7)), 66)
(((6, 1), (5, 7)), 66)
(((3, 2), (2, 9)), 66)
(((3, 3), (2, 9)), 66)

4 14
(((2, 1), (2, 9)), 68)
(((5, 2), (4, 7)), 68)
(((5, 1), (5, 7)), 68)
(((5, 3), (3, 7)), 68)

5 12
(((5, 3), (4, 7)), 70)
(((1, 2), (1, 9)), 70)
(((1, 2), (2, 9)), 70)
(((5, 2), (5, 7)), 70)
(((4, 3), (3, 7)), 70)

17 22
(((3, 1), (5, 7)), 72)
(((1, 3), (1, 9)), 72)
(((3, 3), (4, 7)), 72)
(((3, 2), (3, 7)), 72)
(((3, 1), (4, 7)), 72)
(((3, 3), (3, 7)), 72)
(((3, 1), (3, 7)), 72)
(((4, 3), (4, 7)), 72)
(((3, 2), (4, 7)), 72)
(((2, 1), (5, 7)), 72)
(((3, 3), (5, 7)), 72)
(((5, 3), (5, 7)), 72)
(((2, 1), (3, 7)), 72)
(((1, 3), (2, 9)), 72)
(((4, 3), (5, 7)), 72)
(((2, 1), (4, 7)), 72)
(((3, 2), (5, 7)), 72)

3 4
(((1, 2), (3, 7)), 74)
(((1, 2), (4, 7)), 74)
(((1, 2), (5, 7)), 74)

3 3
(((1, 3), (4, 7)), 76)
(((1, 3), (5, 7)), 76)
(((1, 3), (3, 7)), 76)



*/
