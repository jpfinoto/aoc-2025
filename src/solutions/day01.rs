use crate::aoc::*;
use derive_solution::{parser, solution};

pub struct Input {
    rotations: Vec<i64>,
}

#[parser]
fn parse_input(input: &PuzzleInput) -> Input {
    Input {
        rotations: input
            .get_lines()
            .filter(|line| !line.is_empty())
            .map(|line| match line.chars().next().unwrap() {
                'L' => -line[1..].parse::<i64>().unwrap(),
                'R' => line[1..].parse().unwrap(),
                c => panic!("{c} is not a direction"),
            })
            .collect(),
    }
}

#[solution(day = 1, part = 1)]
fn solve_part_1(input: Input) -> i64 {
    input
        .rotations
        .into_iter()
        .fold((50, 0), |(current, zero_count), x| {
            let new_value = (current + x) % 100;
            if new_value == 0 {
                (new_value, zero_count + 1)
            } else {
                (new_value, zero_count)
            }
        })
        .1
}

#[solution(day = 1, part = 2)]
fn solve_part_2(input: Input) -> i64 {
    input
        .rotations
        .into_iter()
        .fold((50, 0), |(current, zero_count), x| {
            let new_value = current + x;

            // this is for "over rotation" when one step is more than 100 steps
            let mut zero_crossings = (new_value / 100).abs();

            // this checks for sign change
            if current != 0 && new_value.signum() != current.signum() {
                zero_crossings += 1;
            }

            // println!("{current} + {x} => {new_value}, {zero_crossings} crossings");

            (new_value % 100, zero_count + zero_crossings)
        })
        .1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_test;

    const TEST_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part_1() {
        aoc_test!(1, 1, 3, TEST_INPUT);
    }

    #[test]
    fn test_part_2() {
        aoc_test!(1, 2, 6, TEST_INPUT);
    }
}
