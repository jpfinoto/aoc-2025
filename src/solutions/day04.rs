use crate::aoc::*;
use crate::utils::grid::{DenseGrid, XY};
use derive_solution::{parser, solution};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CellState {
    PaperRoll,
    Empty,
}

#[parser]
fn parse_input(input: &PuzzleInput) -> DenseGrid<CellState> {
    DenseGrid::from_rows(
        input
            .get_lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => CellState::Empty,
                        '@' => CellState::PaperRoll,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect(),
    )
}

#[solution(day = 4, part = 1)]
fn solve_part_1(input: DenseGrid<CellState>) -> usize {
    get_removable_paper_rolls(&input).count()
}

#[solution(day = 4, part = 2)]
fn solve_part_2(mut input: DenseGrid<CellState>) -> usize {
    let mut total_removed = 0;
    loop {
        let removable_rolls: Vec<_> = get_removable_paper_rolls(&input).collect();

        if removable_rolls.is_empty() {
            break;
        }

        total_removed += removable_rolls.len();
        removable_rolls.into_iter().for_each(|pos| {
            input.set_at(pos, CellState::Empty);
        });
    }

    total_removed
}

fn get_removable_paper_rolls(grid: &DenseGrid<CellState>) -> impl Iterator<Item = XY> + '_ {
    grid.items()
        .filter(|&(pos, state)| {
            if matches!(state, CellState::PaperRoll) {
                let paper_roll_neighbours = pos
                    .all_neighbours()
                    .filter(|pos2| matches!(grid.at(*pos2), Some(CellState::PaperRoll)))
                    .count();

                paper_roll_neighbours < 4
            } else {
                false
            }
        })
        .map(|(pos, _)| pos)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_test;

    const TEST_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part_1() {
        aoc_test!(4, 1, 13, TEST_INPUT);
    }

    #[test]
    fn test_part_2() {
        aoc_test!(4, 2, 43, TEST_INPUT);
    }
}
