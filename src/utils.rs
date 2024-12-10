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

pub struct Task<'a, Out>
where
    Out: std::fmt::Display,
{
    pub func: fn(&str) -> Out,
    pub examples: &'a [&'a str],
}

impl<'a, Out> Task<'a, Out>
where
    Out: std::fmt::Display,
{
    fn run(&self, filename: &str) -> Out {
        (self.func)(filename)
    }

    pub const fn new(examples: &'a [&'a str], func: fn(&str) -> Out) -> Self {
        Self { func, examples }
    }
}

pub struct Day<'a, Out1, Out2>
where
    Out1: std::fmt::Display,
    Out2: std::fmt::Display,
{
    pub day: usize,

    pub part_1: Task<'a, Out1>,
    pub part_2: Task<'a, Out2>,
}

impl<Out1, Out2> Day<'_, Out1, Out2>
where
    Out1: std::fmt::Display,
    Out2: std::fmt::Display,
{
    fn run_part<Out: std::fmt::Display>(&self, part: &Task<Out>, file: &str) -> Out {
        part.run(&format!("./inputs/day_{:02}/{}", self.day, file))
    }

    pub fn run_example_1(&self, n: usize) -> Out1 {
        self.run_part(&self.part_1, self.part_1.examples[n])
    }

    pub fn run_task_1(&self) -> Out1 {
        self.run_part(&self.part_1, "task.txt")
    }

    pub fn run_example_2(&self, n: usize) -> Out2 {
        self.run_part(&self.part_2, self.part_2.examples[n])
    }

    pub fn run_task_2(&self) -> Out2 {
        self.run_part(&self.part_2, "task.txt")
    }
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
        let res = self.run_task_1();
        println!("{res}");
    }

    fn run_part_2(&self) {
        let res = self.run_task_2();
        println!("{res}");
    }
}
