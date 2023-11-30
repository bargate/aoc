use crate::Runner;


#[derive(Default)]
pub struct Aoc2023_01 {}

impl Aoc2023_01 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_01 {
    fn name(&self) -> (usize, usize) {
        (2023, 1)
    }

    fn parse(&mut self) {
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output("part 1 answer")
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("part 2 answer")
    }
}
