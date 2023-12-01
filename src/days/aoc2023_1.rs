use crate::{find_occurrence_indices, parse::open_input, Runner};

#[derive(Default)]
pub struct Aoc2023_01 {
    lines: Vec<String>,
}

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
        self.lines = open_input("input/2023_01.txt")
            .lines()
            .map(|l| l.to_string())
            .collect();
    }

    fn part1(&mut self) -> Vec<String> {
        let result: u32 = self
            .lines
            .iter()
            .map(|l| {
                let nums = l
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .map(|c| c as u8 - b'0')
                    .collect::<Vec<_>>();
                nums.first().unwrap() * 10 + nums.last().unwrap()
            })
            .fold(0, |a, b| a as u32 + b as u32);
        crate::output(result)
    }

    fn part2(&mut self) -> Vec<String> {
        let valid_nums = [
            "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven",
            "7", "eight", "8", "nine", "9",
        ];
        let result = self
            .lines
            .iter()
            .map(|l| {
                let occurrences = find_occurrence_indices(l, &valid_nums);
                let first = occurrences.first().unwrap();
                let last = occurrences.last().unwrap();

                let first_digit = match first % 2 {
                    0 => valid_nums[*first + 1].parse::<u32>().unwrap(),
                    _ => valid_nums[*first + 1 / 2].parse::<u32>().unwrap(),
                };

                let last_digit = match last % 2 {
                    0 => valid_nums[*last + 1].parse::<u32>().unwrap(),
                    _ => valid_nums[*last + 1 / 2].parse::<u32>().unwrap(),
                };
                let result = first_digit * 10 + last_digit;
                result
            })
            .fold(0, |a, b| a + b);
        crate::output(result)
    }
}
