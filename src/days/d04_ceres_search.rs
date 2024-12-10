use crate::utils::{Day, Task, read_lines};

fn p1_count_xmas(filename: &str) -> usize {
    let cs: Vec<Vec<_>> = read_lines(filename).map(|line| line.chars().collect()).collect();
    let (w, h) = (cs[0].len(), cs.len());
    let get_char = |(x, y): (usize, usize)| cs[y][x];

    let mut res = 0;

    cs.iter().for_each(|row| {
        res += count(row.iter().copied());
        res += count(row.iter().rev().copied());
    });

    (0..w).for_each(|x| {
        let down = (0..h).map(move |y| (x, y)).map(get_char);
        let up = (0..h).rev().map(move |y| (x, y)).map(get_char);
        res += count(up) + count(down)
    });

    let hs = (0..w).flat_map(|x| [(x, 0), (x, h - 1)]);
    let ws = (1..h - 1).flat_map(|y| [(0, y), (w - 1, y)]);
    std::iter::chain(hs, ws).for_each(|(x0, y0)| {
        use std::iter::zip;
        let dl = zip((0..=x0).rev(), y0..h).map(get_char);
        let dr = zip(x0..w, y0..h).map(get_char);
        let ul = zip((0..=x0).rev(), (0..=y0).rev()).map(get_char);
        let ur = zip(x0..w, (0..=y0).rev()).map(get_char);
        res += count(dl) + count(dr) + count(ul) + count(ur);
    });

    res
}

fn p2_cross(filename: &str) -> usize {
    const MS: (char, char) = ('M', 'S');
    let cs: Vec<Vec<_>> = read_lines(filename).map(|line| line.chars().collect()).collect();
    let (w, h) = (cs[0].len(), cs.len());

    (1..h - 1)
        .flat_map(|y| (1..w - 1).map(move |x| (x, y)))
        .filter(|&(x, y)| cs[y][x] == 'A')
        .map(|(x, y)| [cs[y - 1][x - 1], cs[y + 1][x + 1], cs[y - 1][x + 1], cs[y + 1][x - 1]])
        .filter(|&[c11, c12, c21, c22]| {
            ((c11, c12) == MS || (c12, c11) == MS) && ((c21, c22) == MS || (c22, c21) == MS)
        })
        .count()
}

fn count(chars: impl Iterator<Item = char>) -> usize {
    const STR: &str = "XMAS";
    const LEN: usize = STR.len();

    let (_, res) =
        chars.fold((0, 0), |(seek, count), c| match STR.chars().position(|c1| c1 == c) {
            Some(0) => (1, count),
            Some(p) if p == seek => (p + 1 % LEN, count + (p + 1) / LEN),
            _ => (0, count),
        });
    res
}

pub const SOLUTION: Day<usize, usize> = Day {
    day: 4,
    part_1: Task {
        examples: &[
            "example_0_letters.txt",
            "example_0_dots_p1.txt",
            "example_1_letters.txt",
            "example_1_dots_p1.txt",
        ],
        func: p1_count_xmas,
    },
    part_2: Task { examples: &["example_0_letters.txt", "example_0_dots_p2.txt"], func: p2_cross },
};

#[cfg(test)]
mod playground {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    #[ignore]
    fn playground() {}
}

#[cfg(test)]
mod d04_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        let res = SOLUTION.run_example_1(0);
        assert_eq!(res, 18);
    }

    #[test]
    fn p2_example_test() {
        let res = SOLUTION.run_example_2(0);
        assert_eq!(res, 9);
    }
}
