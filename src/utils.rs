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

pub struct Task<'a, Out: std::fmt::Display> {
    pub examples: &'a [&'a str],
    pub task: &'a str,
    pub func: fn(&str) -> Out,
}

impl<Out: std::fmt::Display> Task<'_, Out> {
    pub fn run_example(&self, n: usize) -> Out {
        (self.func)(self.examples[n])
    }

    pub fn run_task(&self) {
        let res = (self.func)(self.task);
        println!("{res}");
    }
}

pub struct Day<'a, Out1, Out2>
where
    Out1: std::fmt::Display,
    Out2: std::fmt::Display,
{
    pub part_1: Task<'a, Out1>,
    pub part_2: Task<'a, Out2>,
}

pub trait Solution {
    fn run_part_1(&self);
    fn run_part_2(&self);
}

impl<Out1, Out2> Solution for Day<'_, Out1, Out2>
where
    Out1: std::fmt::Display,
    Out2: std::fmt::Display,
{
    fn run_part_1(&self) {
        self.part_1.run_task();
    }

    fn run_part_2(&self) {
        self.part_2.run_task();
    }
}
