use crate::aoc::*;
use crate::solution;
use itertools::Itertools;
use std::ops::RangeInclusive;

const DAY: Day = 2;

solution!(DAY, solve_part_1, solve_part_2);

fn solve_part_1(input: impl Lines) -> i64 {
    let ranges = parse_ranges(&input);

    fn find_invalid_numbers_in_range(range: RangeInclusive<i64>) -> impl Iterator<Item = i64> {
        range.filter(|n| is_invalid(*n))
    }

    fn is_invalid(n: i64) -> bool {
        let number_of_digits = n.ilog10() + 1;
        // only a number with an even number of digits can be split into two equal halves
        if number_of_digits.is_multiple_of(2) {
            // divide into a high and low half using powers of 10; it's faster than using strings
            let divisor = 10i64.pow(number_of_digits / 2);
            let high_half = n / divisor;
            let low_half = n % divisor;
            high_half == low_half
        } else {
            false
        }
    }

    ranges.flat_map(find_invalid_numbers_in_range).sum()
}

fn solve_part_2(input: impl Lines) -> i64 {
    let ranges = parse_ranges(&input);

    fn find_invalid_numbers_in_range(range: RangeInclusive<i64>) -> impl Iterator<Item = i64> {
        range.filter(|n| is_invalid(*n))
    }

    fn is_invalid(n: i64) -> bool {
        let digits = n.to_string();
        let len = digits.len();

        // iterates through every possible pattern length, then checks that every nth possible
        // repetition candidate (slot) matches the pattern
        (1..=len / 2)
            .filter(|pattern_len| len.is_multiple_of(*pattern_len))
            .map(|pattern_len| &digits[0..pattern_len])
            .any(|pattern| {
                (1..(len / pattern.len())).all(|slot| {
                    pattern == &digits[slot * pattern.len()..(slot + 1) * pattern.len()]
                })
            })
    }

    ranges.flat_map(find_invalid_numbers_in_range).sum()
}

fn parse_ranges(input: &impl Lines) -> impl Iterator<Item = RangeInclusive<i64>> {
    input.get_raw().trim().split(',').map(|range| {
        let (min, max) = range
            .split('-')
            .map(|x| x.parse::<i64>().unwrap())
            .next_tuple()
            .unwrap();

        min..=max
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_test;

    const TEST_INPUT: &str = concat!(
        "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,",
        "1698522-1698528,446443-446449,38593856-38593862,565653-565659,",
        "824824821-824824827,2121212118-2121212124"
    );

    #[test]
    fn test_part_1() {
        aoc_test!(DAY, 1, 1227775554, TEST_INPUT);
    }

    #[test]
    fn test_part_2() {
        aoc_test!(DAY, 2, 4174379265i64, TEST_INPUT);
    }
}
