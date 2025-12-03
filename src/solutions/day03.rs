use crate::aoc::*;
use derive_solution::{parser, solution};
use itertools::Itertools;

pub struct Input {
    banks: Vec<Vec<u8>>,
}

#[parser]
fn parse_input(input: &PuzzleInput) -> Input {
    Input {
        banks: input
            .get_lines()
            .filter(|l| !l.is_empty())
            .map(|l| {
                l.chars()
                    .map(|x| x.to_digit(10).expect("invalid digit") as u8)
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

/// Returns the maximum joltage for n batteries in that bank.
///
/// The idea behind this method is to create a buffer of the final size,
/// then add one number at a time to this buffer. After adding a number, we
/// delete any number that has a larger number immediately following it, as long
/// as we have enough numbers in the buffer to form a solution.
fn max_number_of_size(bank: &[u8], size: usize) -> i64 {
    let mut slots = bank[..size].to_vec();

    // it's faster to add a number and then remove than to initialise slots with the full list
    for &n in &bank[size..] {
        slots.push(n);
        loop {
            if slots.len() == size || !try_remove_lowest(&mut slots) {
                break;
            }
        }
    }

    // convert the number to base 10
    slots[0..size]
        .iter()
        .cloned()
        .map(i64::from)
        .reduce(|x, x1| x * 10 + x1)
        .unwrap()
}

/// removes the leftmost slot immediately followed by a higher number
fn try_remove_lowest(slots: &mut Vec<u8>) -> bool {
    for (i, (n, next_n)) in slots.iter().tuple_windows().enumerate() {
        if next_n > n {
            slots.remove(i);
            return true;
        }
    }

    false
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
