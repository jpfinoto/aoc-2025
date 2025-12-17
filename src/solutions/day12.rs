use crate::aoc::*;
use crate::utils::grid::XY;
use derive_solution::{parser, solution};
use itertools::Itertools;

#[derive(Clone, Debug)]
#[allow(unused)]
pub struct Input {
    presents: Vec<Present>,
    scenarios: Vec<Scenario>,
}

#[derive(Clone, Debug)]
pub struct Scenario {
    size_x: u32,
    size_y: u32,
    present_counts: Vec<usize>,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Pattern {
    cells: [bool; 9],
    filled_coords: Vec<XY>,
}

#[derive(Clone, Debug)]
#[allow(unused)]
pub struct Present {
    area: u32,
    variants: Vec<Pattern>,
}

#[solution(day = 12, part = 1)]
fn solve_part_1(
    Input {
        presents,
        scenarios,
    }: Input,
) -> usize {
    // println!("{presents:?}");
    // println!("{scenarios:?}");

    scenarios
        .iter()
        .filter(|s| {
            let total_area = s.size_x * s.size_y;
            let max_present_area = (s.present_counts.iter().sum::<usize>() as u32) * 9;

            total_area >= max_present_area
        })
        .count()
}

#[solution(day = 12, part = 2)]
fn solve_part_2(_input: Input) -> i64 {
    0
}

impl Pattern {
    fn new(cells: [bool; 9]) -> Self {
        let filled_coords = (0..3)
            .cartesian_product(0..3)
            .filter(|(x, y)| cells[(x + y * 3) as usize])
            .map(|(x, y)| XY { x, y })
            .collect();

        Pattern {
            cells,
            filled_coords,
        }
    }
}

impl Present {
    fn new(base_pattern: Pattern) -> Self {
        let area = base_pattern.cells.iter().filter(|&&cell| cell).count() as u32;

        Present {
            area,
            variants: vec![base_pattern],
        }
    }
}

#[parser]
fn parse_input(input: &PuzzleInput) -> Input {
    let mut line_iter = input.get_lines();

    let presents: Vec<Present> = std::iter::from_fn({
        || {
            line_iter
                .by_ref()
                .find(|line| !line.is_empty())?
                .strip_suffix(':')?
                .parse::<u32>()
                .expect("present id must be a number");

            let cells = line_iter
                .by_ref()
                .take(3)
                .flat_map(str::chars)
                .map(|c| c == '#')
                .collect_vec()
                .try_into()
                .unwrap();

            Some(Present::new(Pattern::new(cells)))
        }
    })
    .collect();

    let scenarios = input
        .get_lines()
        .skip(5 * presents.len())
        .filter(|line| !line.is_empty() && line.contains(':'))
        .map(|line| {
            let (size_part, counts_part) = line.split_once(':').unwrap();
            let (size_x, size_y) = size_part.split_once('x').unwrap();

            let present_counts: Vec<usize> = counts_part
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            assert_eq!(present_counts.len(), presents.len());

            Scenario {
                size_x: size_x.parse().unwrap(),
                size_y: size_y.parse().unwrap(),
                present_counts,
            }
        })
        .collect();

    Input {
        presents,
        scenarios,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_test;

    const TEST_INPUT: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test]
    fn test_part_1() {
        aoc_test!(12, 1, 3, TEST_INPUT);
    }

    #[test]
    fn test_part_2() {
        aoc_test!(12, 2, 6, TEST_INPUT);
    }
}
