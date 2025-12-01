use crate::aoc::*;
use crate::solution;

const DAY: Day = 1;

solution!(DAY, solve_part_1, solve_part_2);

fn solve_part_1(input: impl Lines) -> i64 {
    let numbers = get_numbers(&input);

    numbers
        .fold((50, 0), |(current, zero_count), x| {
            let new_value = (current + x) % 100;
            (
                new_value,
                if new_value == 0 {
                    zero_count + 1
                } else {
                    zero_count
                },
            )
        })
        .1
}

fn solve_part_2(input: impl Lines) -> i64 {
    let numbers = get_numbers(&input);

    numbers
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

fn get_numbers(input: &impl Lines) -> impl Iterator<Item = i64> {
    input
        .get_lines()
        .filter(|line| !line.is_empty())
        .map(|line| match line.chars().next().unwrap() {
            'L' => -line[1..].parse::<i64>().unwrap(),
            'R' => line[1..].parse().unwrap(),
            c => panic!("{c} is not a direction"),
        })
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
        aoc_test!(DAY, 1, 3, TEST_INPUT);
    }

    #[test]
    fn test_part_2() {
        aoc_test!(DAY, 2, 6, TEST_INPUT);
    }
}
