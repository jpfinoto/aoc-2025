use crate::aoc::*;
use derive_solution::{parser, solution};
use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Range {
    start: i64,
    end: i64,
}

#[derive(Debug, Clone)]
pub struct Input {
    fresh_ranges: Vec<Range>,
    ingredients: Vec<i64>,
}

impl Range {
    fn contains(&self, n: i64) -> bool {
        n >= self.start && n <= self.end
    }
}

#[parser]
fn parse_input(input: &PuzzleInput) -> Input {
    let mut lines = input.get_lines();
    let fresh_ranges = (&mut lines)
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (start, end) = line
                .split('-')
                .map(|n| n.parse().unwrap())
                .next_tuple()
                .unwrap();
            Range { start, end }
        })
        .collect();

    let ingredients = lines.map(|line| line.parse().unwrap()).collect();

    Input {
        fresh_ranges,
        ingredients,
    }
}

#[solution(day = 5, part = 1)]
fn solve_part_1(mut input: Input) -> usize {
    input.fresh_ranges.sort_by_key(|r| r.start);

    let ranges = get_non_overlapping_ranges(&input.fresh_ranges);

    input
        .ingredients
        .into_iter()
        .filter(|n| {
            match ranges.binary_search_by_key(n, |range| range.start) {
                Ok(_) => {
                    // the unlikely scenario where we find it exactly
                    true
                }
                Err(i) if i > 0 => ranges[i - 1].contains(*n),
                Err(_) => false,
            }
        })
        .count()
}

#[solution(day = 5, part = 2)]
fn solve_part_2(mut input: Input) -> i64 {
    input.fresh_ranges.sort_by_key(|r| r.start);

    get_non_overlapping_ranges(&input.fresh_ranges)
        .iter()
        .fold(0, |acc, range| acc + (range.end - range.start + 1))
}

fn get_non_overlapping_ranges(ranges: &[Range]) -> Vec<Range> {
    ranges.iter().fold(vec![], |mut processed, range| {
        if let Some(last) = processed.last_mut() {
            if range.start <= last.end {
                last.end = range.end.max(last.end);
            } else if range.end > last.end {
                processed.push(range.clone());
            }
        } else {
            processed.push(range.clone());
        }

        processed
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_test;

    const TEST_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part_1() {
        aoc_test!(5, 1, 3, TEST_INPUT);
    }

    #[test]
    fn test_part_2() {
        aoc_test!(5, 2, 14, TEST_INPUT);
    }
}
