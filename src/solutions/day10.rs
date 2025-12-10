use crate::aoc::*;
use derive_solution::{parser, solution};
use itertools::Itertools;
use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;
use std::iter;
use std::ops::RangeInclusive;

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

#[solution(day = 10, part = 2, unsolved)]
fn solve_part_2(Input(machines): Input) -> usize {
    machines
        .iter()
        .flat_map(|machine| {
            let min_budget = *machine.joltage.iter().max().unwrap() as usize;
            (min_budget..).find_map(|budget| try_solve_joltage(machine, budget).map(|_| budget))
        })
        .sum()
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

fn try_solve_joltage(machine: &Machine, budget: usize) -> Option<Vec<usize>> {
    println!("Trying budget {budget}");

    CombinationIterator::new(
        machine.button_indices.len(),
        budget,
        Some(|partial: &[usize]| -> bool {
            is_partial_combination_possible(machine, partial, budget)
        }),
    )
    // .par_bridge()
    .find(|combination| {
        let values = calc_joltage(machine, combination);

        values == machine.joltage
    })
}

fn calc_joltage(machine: &Machine, combination: &[usize]) -> Vec<u32> {
    let mut values = vec![0; machine.joltage.len()];

    for (times, button_indices) in combination.iter().zip(machine.button_indices.iter()) {
        for &joltage_index in button_indices {
            values[joltage_index as usize] += *times as u32;
        }
    }
    values
}

fn is_partial_combination_possible(
    machine: &Machine,
    combination: &[usize],
    total_budget: usize,
) -> bool {
    let values = calc_joltage(machine, combination);
    let used_budget = combination.iter().sum::<usize>();

    let leftover = values
        .iter()
        .zip_eq(machine.joltage.iter())
        .map(|(value, expected_value)| (*expected_value as i32) - (*value as i32))
        .collect_vec();

    leftover.iter().all(|x| *x >= 0)
        && (total_budget - used_budget) >= (*machine.joltage.iter().max().unwrap() as usize)
}

#[derive(Debug, Clone)]
struct CombinationIterator<F: Fn(&[usize]) -> bool> {
    budget: usize,
    items: Vec<Option<CombinationState>>,
    partial_checker: Option<F>,
}

#[derive(Debug, Clone)]
struct CombinationState {
    budget: usize,
    range: RangeInclusive<usize>,
    last_value: Option<usize>,
}

impl<F: Fn(&[usize]) -> bool> Iterator for CombinationIterator<F> {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        // go over the iterators and "refill" any that need refilling
        let mut needs_refill = true;
        while needs_refill {
            needs_refill = false;
            for i in 0..self.items.len() {
                if matches!(&self.items[i], None) {
                    if i == 0 {
                        return None;
                    }

                    // try to refill and backtrack if the previous one exhausts
                    if let Some(_) = self.items[i - 1].as_mut()?.next() {
                        self.items[i] = Some(CombinationState::new(
                            self.budget
                                - &self.items[0..i]
                                    .iter()
                                    .flatten()
                                    .map(|state| state.last_value.unwrap())
                                    .sum(),
                        ));
                    } else {
                        self.items[i - 1] = None;
                        needs_refill = true;
                        break;
                    }
                }

                if i > 0 {
                    let partial_result: Vec<usize> = self.items[0..=i]
                        .iter()
                        .flat_map(|el| el.as_ref().unwrap().last_value)
                        .collect();

                    if let Some(checker) = &self.partial_checker
                        && !partial_result.is_empty()
                        && !checker(&partial_result)
                    {
                        self.items[i..].iter_mut().for_each(|el| *el = None);
                        needs_refill = true;
                        break;
                    }
                }
            }

            if !needs_refill && matches!(self.items.last_mut()?.as_mut()?.next(), None) {
                *self.items.last_mut()? = None;
                needs_refill = true;
            }
        }

        let total: usize = self
            .items
            .iter()
            .flatten()
            .flat_map(|state| state.last_value)
            .sum();

        Some(
            self.items
                .iter()
                .flat_map(|el| el.as_ref().unwrap().last_value)
                .chain(iter::once(self.budget - total))
                .collect(),
        )
    }
}

impl<F: Fn(&[usize]) -> bool> CombinationIterator<F> {
    fn new(n: usize, budget: usize, partial_checker: Option<F>) -> Self {
        assert!(n >= 2);
        CombinationIterator {
            partial_checker,
            budget,
            items: iter::once(Some(CombinationState::new(budget)))
                .chain(iter::repeat_n(None, n - 2))
                .collect(),
        }
    }
}

impl CombinationState {
    fn new(budget: usize) -> Self {
        CombinationState {
            budget,
            range: 0..=budget,
            last_value: None,
        }
    }

    fn next(&mut self) -> Option<usize> {
        let value = self.budget - self.range.next()?;
        self.last_value = Some(value);
        Some(value)
    }
}

lazy_static! {
    static ref LINE_REGEX: Regex =
        Regex::new(r"^\[(?<lights>[.#]+)] \((?<buttons>[^{]*)\) \{(?<joltage>.*)}$").unwrap();
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
    fn test_combination_iterator() {
        let mut iter = CombinationIterator::new(3, 5, None::<fn(&[usize]) -> bool>);
        assert_eq!(iter.next(), Some(vec![5, 0, 0]));
        assert_eq!(iter.next(), Some(vec![4, 1, 0]));
        assert_eq!(iter.next(), Some(vec![4, 0, 1]));

        for values in CombinationIterator::new(3, 5, None::<fn(&[usize]) -> bool>) {
            println!("{values:?}");
        }
    }
}
