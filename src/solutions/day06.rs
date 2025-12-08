//! Day 6: I hate this day so much.
//! It's dumb. I hate it.

use crate::aoc::*;
use derive_solution::{parser, solution};
use itertools::Itertools;
use std::iter;

pub struct Column {
    numbers: Vec<i64>,
    operation: Operation,
}

const MAX_DIGITS: usize = 4;

pub struct CephalopodColumn {
    numbers: Vec<i64>,
    operation: Operation,
}

pub enum Operation {
    Add,
    Multiply,
}

#[solution(day = 6, part = 1)]
fn solve_part_1(columns: Vec<Column>) -> i64 {
    columns
        .into_iter()
        .map(|col| match col.operation {
            Operation::Add => col.numbers.iter().sum::<i64>(),
            Operation::Multiply => col.numbers.iter().product(),
        })
        .sum()
}

#[solution(day = 6, part = 2)]
fn solve_part_2(mut columns: Vec<CephalopodColumn>) -> i64 {
    columns.reverse();

    columns
        .into_iter()
        .map(|col| match col.operation {
            Operation::Add => col.numbers.into_iter().sum::<i64>(),
            Operation::Multiply => col.numbers.into_iter().product(),
        })
        .sum()
}

fn decode_cephalopod_column(column: &[[Option<u8>; MAX_DIGITS]]) -> Vec<i64> {
    (0..MAX_DIGITS)
        .map(move |digit_index| {
            column.iter().map(|n| n[digit_index]).fold(0i64, |acc, n| {
                if let Some(n) = n {
                    acc * 10 + (n as i64)
                } else {
                    acc
                }
            })
        })
        .filter(|n| *n != 0)
        .collect()
}

#[parser]
fn parse_input(input: &PuzzleInput) -> Vec<Column> {
    let lines = input
        .get_lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split_whitespace().collect_vec())
        .collect_vec();

    let num_columns = lines[0].len();
    let num_rows = lines.len();

    (0..num_columns)
        .map(|i| Column {
            numbers: lines[0..num_rows - 1]
                .iter()
                .map(|row| row[i].parse().unwrap())
                .collect_vec(),
            operation: match lines[num_rows - 1][i] {
                "*" => Operation::Multiply,
                "+" => Operation::Add,
                _ => unreachable!(),
            },
        })
        .collect()
}

#[parser]
fn parse_input_part_2(input: &PuzzleInput) -> Vec<CephalopodColumn> {
    let last_line = input
        .get_raw()
        .split('\n')
        .filter(|line| !line.trim().is_empty())
        .next_back()
        .unwrap();

    let mut widths = last_line
        .split(['*', '+'])
        .skip(1)
        .map(|s| s.len())
        .collect_vec();
    *widths.last_mut().unwrap() += 1;

    let line_starts = widths
        .iter()
        .scan(0, |acc, width| {
            let initial_offset = *acc;
            // + 1 to account for the column spacing
            *acc += width + 1;
            Some(initial_offset)
        })
        .collect_vec();

    let operations = last_line.split_whitespace().collect_vec();
    assert_eq!(operations.len(), widths.len());

    let num_columns = operations.len();
    let num_rows = input.get_lines().count();

    let lines = input
        .get_raw()
        .split('\n')
        .take(num_rows - 1)
        .map(|line| {
            line_starts
                .iter()
                .zip_eq(widths.iter())
                .map(|(start, width)| {
                    let values: [Option<u8>; MAX_DIGITS] = line[*start..start + width]
                        .chars()
                        .chain(iter::repeat(' '))
                        .take(MAX_DIGITS)
                        .map(|c| c.to_digit(10).map(|d| d as u8))
                        .collect_vec()
                        .try_into()
                        .unwrap();

                    values
                })
                .collect_vec()
        })
        .collect_vec();

    (0..num_columns)
        .map(|i| CephalopodColumn {
            numbers: decode_cephalopod_column(&lines.iter().map(|line| line[i]).collect_vec()),
            operation: match operations[i] {
                "*" => Operation::Multiply,
                "+" => Operation::Add,
                _ => unreachable!(),
            },
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_test;

    const TEST_INPUT: &str = concat!(
        "123 328  51 64 \n",
        " 45 64  387 23 \n",
        "  6 98  215 314\n",
        "*   +   *   +  "
    );

    #[test]
    fn test_part_1() {
        aoc_test!(6, 1, 4277556, TEST_INPUT);
    }

    #[test]
    fn test_part_2() {
        aoc_test!(6, 2, 3263827, TEST_INPUT);
    }

    #[test]
    fn test_part_2_single_columns() {
        let input = "1 2 3\n4 5 6\n+ * +";
        aoc_test!(6, 2, 14 + 25 + 36, input);
    }
}
