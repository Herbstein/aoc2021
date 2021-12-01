use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day1)]
pub fn parse(input: &str) -> Vec<usize> {
    input.lines().flat_map(|s| s.parse::<usize>()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    input
        .windows(2)
        .map(|window| if window[0] < window[1] { 1 } else { 0 })
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    input
        .windows(3)
        .map(|window| window.iter().sum::<usize>())
        .tuple_windows()
        .map(|(prev, next)| if prev < next { 1 } else { 0 })
        .sum()
}

#[aoc(day1, part2, std)]
pub fn solve_part2_std(input: &[usize]) -> usize {
    let mut window_sums = vec![];
    for i in 0..input.len() - 2 {
        window_sums.push(input[i] + input[i + 1] + input[i + 2]);
    }

    solve_part1(&window_sums)
}
