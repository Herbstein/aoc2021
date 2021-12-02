use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

pub enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => return Err(()),
        })
    }
}

pub struct Instruction {
    direction: Direction,
    units: usize,
}

#[derive(Clone, Copy)]
pub struct Position {
    horizontal: usize,
    vertical: usize,
    aim: usize,
}

impl Position {
    pub fn update(self, instruction: &Instruction) -> Self {
        match instruction.direction {
            Direction::Forward => Self {
                horizontal: self.horizontal + instruction.units,
                vertical: self.vertical + (self.aim * instruction.units),
                ..self
            },
            Direction::Down => Self {
                aim: self.aim + instruction.units,
                ..self
            },
            Direction::Up => Self {
                aim: self.aim - instruction.units,
                ..self
            },
        }
    }

    pub fn update_mut(&mut self, instruction: &Instruction) {
        match instruction.direction {
            Direction::Forward => {
                self.horizontal += instruction.units;
                self.vertical += self.aim * instruction.units;
            }
            Direction::Down => {
                self.aim += instruction.units;
            }
            Direction::Up => {
                self.aim -= instruction.units;
            }
        }
    }

    pub fn into_part1_result(self) -> usize {
        self.horizontal * self.aim
    }

    pub fn into_part2_result(self) -> usize {
        self.horizontal * self.vertical
    }
}

impl Default for Position {
    fn default() -> Self {
        Self {
            horizontal: 0,
            vertical: 0,
            aim: 0,
        }
    }
}

#[aoc_generator(day2)]
pub fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .flat_map(|l| {
            let mut split = l.split(' ');
            split
                .next()
                .and_then(|direction| split.next().map(|units| (direction, units)))
                .and_then(|(direction, units)| {
                    direction
                        .parse()
                        .ok()
                        .and_then(|direction| units.parse().ok().map(|units| (direction, units)))
                })
        })
        .map(|(direction, units)| Instruction { direction, units })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(instructions: &[Instruction]) -> usize {
    instructions
        .iter()
        .fold(Position::default(), Position::update)
        .into_part1_result()
}

#[aoc(day2, part1, mutable)]
pub fn solve_part1_mut(instructions: &[Instruction]) -> usize {
    let mut state = Position::default();

    for i in instructions {
        state.update_mut(i);
    }

    state.into_part1_result()
}

#[aoc(day2, part2)]
pub fn solve_part2(instructions: &[Instruction]) -> usize {
    instructions
        .iter()
        .fold(Position::default(), Position::update)
        .into_part2_result()
}

#[aoc(day2, part2, mutable)]
pub fn solve_part2_mut(instructions: &[Instruction]) -> usize {
    let mut state = Position::default();

    for i in instructions {
        state.update_mut(i);
    }

    state.into_part2_result()
}
