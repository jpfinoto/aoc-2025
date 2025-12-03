use crate::aoc::*;
use derive_solution::{parser, solution};
use itertools::Itertools;

pub struct Input {
    banks: Vec<Vec<i64>>,
}

#[parser]
fn parse_input(input: &PuzzleInput) -> Input {
    Input {
        banks: input
            .get_lines()
            .filter(|l| !l.is_empty())
            .map(|l| {
                l.chars()
                    .map(|x| x.to_digit(10).expect("invalid digit") as i64)
                    .collect()
            })
            .collect(),
    }
}

#[solution(day = 3, part = 1)]
fn solve_part_1(input: Input) -> i64 {
    input
        .banks
        .iter()
        .map(|bank| max_number_of_size(bank, 2))
        // .inspect(|n| println!("{n}"))
        .sum()
}

#[solution(day = 3, part = 2)]
fn solve_part_2(input: Input) -> i64 {
    input
        .banks
        .iter()
        .map(|bank| max_number_of_size(bank, 12))
        // .inspect(|n| println!("{n}"))
        .sum()
}

fn max_number_of_size(bank: &[i64], size: usize) -> i64 {
    let mut slots = vec![];
    let mut index = 0;

    for i in 0..size {
        let limit = bank.len() - size + i;

        // println!("search from {i} to {limit}: {:?}", &bank[index..=limit]);

        let (new_index, largest) = bank[index..=limit]
            .iter()
            .enumerate()
            .sorted_by_key(|&(i, n)| (-n, i))
            .next()
            .unwrap();

        // println!("largest: {largest} at index {new_index}");

        slots.push(*largest);
        index += new_index + 1;
    }

    slots
        .iter()
        .rev()
        .enumerate()
        .map(|(i, n)| n * 10i64.pow(i as u32))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_test;

    const TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part_1() {
        aoc_test!(3, 1, 357, TEST_INPUT);
    }

    #[test]
    fn test_part_2() {
        aoc_test!(3, 2, 3121910778619i64, TEST_INPUT);
    }
}
