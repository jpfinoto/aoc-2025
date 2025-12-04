use crate::aoc::*;
use crate::utils::grid::{DenseGrid, XY};
use derive_solution::{parser, solution};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Cell {
    PaperRoll,
    Empty,
}

#[parser]
fn parse_input(input: &PuzzleInput) -> DenseGrid<Cell> {
    DenseGrid::from_rows(
        input
            .get_lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Cell::Empty,
                        '@' => Cell::PaperRoll,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect(),
    )
}

#[solution(day = 4, part = 1)]
fn solve_part_1(grid: DenseGrid<Cell>) -> usize {
    get_removable_paper_rolls(&grid).count()
}

#[solution(day = 4, part = 2)]
fn solve_part_2(mut grid: DenseGrid<Cell>) -> usize {
    let mut total_removed = 0;
    loop {
        let removable_rolls: Vec<_> = get_removable_paper_rolls(&grid).collect();

        if removable_rolls.is_empty() {
            break;
        }

        total_removed += removable_rolls.len();
        removable_rolls.into_iter().for_each(|pos| {
            grid.set_at(pos, Cell::Empty);
        });
    }

    total_removed
}

fn get_removable_paper_rolls(grid: &DenseGrid<Cell>) -> impl Iterator<Item = XY> + '_ {
    grid.items()
        .filter(|&(pos, state)| {
            matches!(state, Cell::PaperRoll) && count_neighbour_paper_rolls(grid, pos) < 4
        })
        .map(|(pos, _)| pos)
}

fn count_neighbour_paper_rolls(grid: &DenseGrid<Cell>, pos: XY) -> usize {
    pos.all_neighbours()
        .filter(|pos2| matches!(grid.at(*pos2), Some(Cell::PaperRoll)))
        .count()
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
