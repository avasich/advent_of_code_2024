use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub fn read_lines<P: AsRef<Path>>(filename: P) -> impl Iterator<Item = String> {
    let file = File::open(filename).expect("error reading file");
    #[allow(clippy::lines_filter_map_ok)]
    io::BufReader::new(file).lines().flatten()
}

#[macro_export]
macro_rules! day {
    ($day:expr,
     part_1: { examples: [$($file1:expr),* $(,)?], func: $func1:expr $(,)? },
     part_2: { examples: [$($file2:expr),* $(,)?], func: $func2:expr $(,)? } $(,)?
    ) => {
        Day {
            day: $day,
            part_1: Task {
                examples: match $day {
                    ..10 => &[$(concat!("./inputs/day_0", stringify!($day), "/", $file1)),*],
                    10.. => &[$(concat!("./inputs/day_", stringify!($day), "/", $file1)),*],
                },
                task: match $day {
                    ..10 => concat!("./inputs/day_0", stringify!($day), "/task.txt"),
                    10.. => concat!("./inputs/day_", stringify!($day), "/task.txt"),
                },
                func: $func1,
            },
            part_2: Task {
                examples: match $day {
                    ..10 => &[$(concat!("./inputs/day_0", stringify!($day), "/", $file2)),*],
                    10.. => &[$(concat!("./inputs/day_", stringify!($day), "/", $file2)),*],
                },
                task: match $day {
                    ..10 => concat!("./inputs/day_0", stringify!($day), "/task.txt"),
                    10.. => concat!("./inputs/day_", stringify!($day), "/task.txt"),
                },
                func: $func2,
        }
        }
    };
}

pub struct Task<'a, Out> {
    pub examples: &'a [&'a str],
    pub task: &'a str,
    pub func: fn(&str) -> Out,
}

impl<Out> Task<'_, Out> {
    pub fn run_example(&self, n: usize) -> Out {
        (self.func)(self.examples[n])
    }

    pub fn run_task(&self) -> Out {
        (self.func)(self.task)
    }
}

pub struct Day<'a, Out1, Out2> {
    pub day: usize,
    pub part_1: Task<'a, Out1>,
    pub part_2: Task<'a, Out2>,
}

pub trait Solution {
    fn run_part_1(&self);
    fn run_part_2(&self);
}

impl<Out1, Out2> Solution for Day<'_, Out1, Out2>
where
    Out1: std::fmt::Debug,
    Out2: std::fmt::Debug,
{
    fn run_part_1(&self) {
        let res = self.part_1.run_task();
        println!("d{:02} p1: {res:?}", self.day);
    }

    fn run_part_2(&self) {
        let res = self.part_2.run_task();
        println!("d{:2} p2: {res:?}", self.day);
    }
}
