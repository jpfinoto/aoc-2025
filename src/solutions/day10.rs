use crate::aoc::*;
use derive_solution::{parser, solution};
use good_lp::{Expression, Solution, SolverModel, constraint, default_solver, variables};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

pub struct Input(Vec<Machine>);

pub struct Machine {
    target_light_states: u32,
    button_masks: Vec<u32>,
    button_indices: Vec<Vec<u8>>,
    joltage: Vec<u32>,
}

#[solution(day = 10, part = 1)]
fn solve_part_1(Input(machines): Input) -> u32 {
    machines.iter().map(min_presses_for_lights).sum()
}

#[solution(day = 10, part = 2)]
fn solve_part_2(Input(machines): Input) -> usize {
    machines.iter().map(try_solve_machine).sum()
}

#[parser]
fn parse_input(input: &PuzzleInput) -> Input {
    Input(input.get_lines().filter_map(parse_machine).collect())
}

fn min_presses_for_lights(machine: &Machine) -> u32 {
    let max_button_mask = 2u32.pow(machine.button_masks.len() as u32) - 1;

    (0..max_button_mask)
        .filter(|&button_mask| {
            let light_state = get_light_state(&machine.button_masks, button_mask);
            light_state == machine.target_light_states
        })
        .map(|state| state.count_ones())
        .min()
        .unwrap()
}

fn get_light_state(buttons: &[u32], state: u32) -> u32 {
    buttons.iter().enumerate().fold(0, |acc, (i, button)| {
        if state & (1 << i) != 0 {
            acc ^ button
        } else {
            acc
        }
    })
}

fn try_solve_machine(machine: &Machine) -> usize {
    let mut vars = variables!();
    let mut total_presses: Expression = 0.into();

    let mut joltages: Vec<Expression> = machine.joltage.iter().map(|_| 0.into()).collect();

    for indices in &machine.button_indices {
        let button_presses = vars.add(good_lp::variable::variable().bounds(0..).integer());
        total_presses += button_presses;

        for &index in indices {
            joltages[index as usize] += button_presses;
        }
    }

    let constraints = joltages
        .into_iter()
        .zip(machine.joltage.iter())
        .map(|(j, &joltage)| constraint!(j == joltage))
        .collect_vec();

    let problem = vars
        .minimise(total_presses.clone())
        .using(default_solver)
        .with_all(constraints);

    let solution = problem.solve().expect("all problems should be solvable");

    solution.eval(total_presses).round() as usize
}

lazy_static! {
    static ref LINE_REGEX: Regex =
        Regex::new(r"^\[(?<lights>[.#]*)] \((?<buttons>[^{]*)\) \{(?<joltage>.*)}$").unwrap();
}

fn parse_machine(line: &str) -> Option<Machine> {
    if let Some(matches) = LINE_REGEX.captures(line) {
        let target_light_states = matches["lights"]
            .chars()
            .map(|c| match c {
                '#' => 1,
                '.' => 0,
                _ => unreachable!(),
            })
            .fold(0, |acc, x| (acc << 1) | x);

        let number_of_lights = matches["lights"].len() as u32;

        let button_indices = matches["buttons"]
            .split(") (")
            .map(|s| s.split(',').map(|s| s.parse::<u8>().unwrap()).collect_vec())
            .collect_vec();

        let button_masks = button_indices
            .iter()
            .map(|indices| {
                indices
                    .iter()
                    .map(|s| 1 << (number_of_lights - (*s as u32) - 1))
                    .fold(0, |acc, x| acc | x)
            })
            .collect_vec();

        let joltage = matches["joltage"]
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect_vec();

        Some(Machine {
            target_light_states,
            button_masks,
            button_indices,
            joltage,
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_test;

    const TEST_INPUT: &str = concat!(
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n",
        "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n",
        "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}\n",
    );

    #[test]
    fn test_part_1() {
        aoc_test!(10, 1, 7, TEST_INPUT);
    }

    #[test]
    fn test_part_2() {
        aoc_test!(10, 2, 33, TEST_INPUT);
    }

    #[test]
    fn test_get_light_state() {
        assert_eq!(get_light_state(&[0b110011], 0b1), 0b110011);

        assert_eq!(get_light_state(&[0b110011, 0b011110], 0b11), 0b101101);
    }

    #[test]
    fn test_machine() {
        let machine = parse_machine("[.#..] (0,1,2) (0,3) (1,2) (3) (0,2) {47,24,44,16}").unwrap();
        assert_eq!(min_presses_for_lights(&machine), 2);
    }

    #[test]
    fn test_solver() {
        let machine =
            parse_machine("[.....] (0,1,2,3) (2,3) (0,1) (2) (0,2,3) (4) {99,99,99,99,1}").unwrap();
        assert_eq!(try_solve_machine(&machine), 100);

        let machine = parse_machine("[.#.#] (0,1,2) (1,3) {0,7,0,7}").unwrap();
        assert_eq!(try_solve_machine(&machine), 7);
    }
}
