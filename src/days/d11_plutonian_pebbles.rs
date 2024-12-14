use crate::{
    day,
    utils::{Day, Task, read_lines},
};

fn p1_naive(filename: &str, blinks: usize) -> usize {
    let ns: Vec<_> = read_lines(filename)
        .next()
        .unwrap()
        .split_whitespace()
        .flat_map(str::parse::<usize>)
        .collect();

    fn split_number(n: usize) -> Option<(usize, usize)> {
        std::iter::successors(Some(10), |&x| Some(10 * x))
            .skip_while(|&x| x * x < n)
            .take_while(|&x| x / 10 <= n / x && n / x < x)
            .find_map(|x| Some((n / x, n % x)))
    }

    std::iter::successors(Some(ns), |ns| {
        let mut res = Vec::with_capacity(ns.len() * 2);
        ns.iter().for_each(|&n| match n {
            0 => res.push(1),
            p => match split_number(p) {
                Some((a, b)) => {
                    // println!("split: {p} = {a} {b}");
                    res.push(a);
                    res.push(b);
                }
                None => res.push(p * 2024),
            },
        });
        Some(res)
    })
    .take(blinks + 1)
    .last()
    .unwrap()
    .len()
}

fn p1(filename: &str) -> usize {
    p1_naive(filename, 25)
}
fn p2(filename: &str) -> usize {
    p1_naive(filename, 75)
}

pub const SOLUTION: Day<usize, usize> = day! { 11,
    part_1: { examples: ["example_2.txt"], func: p1 },
    part_2: { examples: ["example_2.txt"], func: p2 }
};

#[cfg(test)]
mod d11_tests {
    use super::*;
    
    #[test]
    fn p1_example_test() {
        let res = SOLUTION.part_1.run_example(0);
        assert_eq!(res,  55312);
    }

    #[test]
    fn p2_example_test() {}
}

#[cfg(test)]
mod playground {
    use crate::days::d11_plutonian_pebbles::{p1_naive, SOLUTION};

    #[test]
    fn foo() {
        // (10..50000).step_by(10).for_each(|x| println!("{x:>5}: {:?}", split_number(x)));
        
        p1_naive(SOLUTION.part_1.examples[0], 10);
    }
}
