use std::{fmt::Error, str::FromStr};

use crate::{parse::open_input, Runner};

#[derive(Default, Debug)]
struct Entry {
    r: u32,
    g: u32,
    b: u32,
}

#[derive(Debug)]
struct Game {
    id: u8,
    entries: Vec<Entry>,
}

impl FromStr for Entry {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut entry = Self::default();
        s.split(", ").for_each(|c| {
            let (num, color) = c.split_once(" ").unwrap();
            match color {
                "red" => entry.r += num.parse::<u32>().unwrap(),
                "green" => entry.g += num.parse::<u32>().unwrap(),
                "blue" => entry.b += num.parse::<u32>().unwrap(),
                _ => (),
            }
        });
        Ok(entry)
    }
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, rest) = s.split_once(": ").unwrap();
        let id: u8 = id.replace("Game ", "").parse().unwrap();
        let entries: Vec<Entry> = rest
            .split("; ")
            .map(|e| Entry::from_str(e).unwrap())
            .collect();

        return Ok(Self { id, entries });
    }
}

#[derive(Default)]
pub struct Aoc2023_02 {
    games: Vec<Game>,
}

impl Aoc2023_02 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_02 {
    fn name(&self) -> (usize, usize) {
        (2023, 2)
    }

    fn parse(&mut self) {
        self.games = open_input("input/2023_02.txt")
            .lines()
            .map(|l| Game::from_str(l).unwrap())
            .collect::<Vec<_>>();
    }

    fn part1(&mut self) -> Vec<String> {
        let result = self
            .games
            .iter()
            .filter(|&game| {
                game.entries
                    .iter()
                    .all(|entry| entry.r <= 12 && entry.g <= 13 && entry.b <= 14)
            })
            .fold(0, |t, g| t + g.id as u32);
        crate::output(result)
    }

    fn part2(&mut self) -> Vec<String> {
        let power_sum = self
            .games
            .iter()
            .map(|game| {
                let mut r_min = 0;
                let mut g_min = 0;
                let mut b_min = 0;

                game.entries.iter().for_each(|entry| {
                    r_min = r_min.max(entry.r);
                    g_min = g_min.max(entry.g);
                    b_min = b_min.max(entry.b);
                });
                r_min * g_min * b_min
            })
            .fold(0, |t, power| t + power);

        crate::output(power_sum)
    }
}
