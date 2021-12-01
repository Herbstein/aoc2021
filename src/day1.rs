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
        .filter(|window| window[0] < window[1])
        .count()
}

#[aoc(day1, part1, forr)]
pub fn solve_part1_for(input: &[usize]) -> usize {
    let mut count = 0;
    for i in 0..input.len() - 1 {
        if input[i] < input[i + 1] {
            count += 1;
        }
    }
    count
}

#[aoc(day1, part1, for_rev)]
pub fn solve_part1_for_rev(input: &[usize]) -> usize {
    let mut count = 0;
    for i in 0..input.len() - 1 {
        if input[i + 1] > input[i] {
            count += 1;
        }
    }
    count
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

#[aoc(day1, part2, comparison)]
pub fn solve_part2_comparison(input: &[usize]) -> usize {
    input
        .iter()
        .zip(input.iter().skip(3))
        .map(|(prev, next)| if prev < next { 1 } else { 0 })
        .sum()
}

#[aoc(day1, part2, comparison_std)]
pub fn solve_part2_comparison_std(input: &[usize]) -> usize {
    let mut count = 0;
    for i in 0..input.len() - 3 {
        if input[i] < input[i + 3] {
            count += 1;
        }
    }

    count
}
