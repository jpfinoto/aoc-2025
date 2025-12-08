use crate::aoc::*;
use crate::utils::grid::{DenseGrid, XY};
use derive_solution::{parser, solution};
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Cell {
    Empty,
    Source,
    Splitter,
    Beam,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Source => write!(f, "S"),
            Cell::Splitter => write!(f, "^"),
            Cell::Beam => write!(f, "|"),
        }
    }
}

#[parser]
fn parse_input(input: &PuzzleInput) -> DenseGrid<Cell> {
    DenseGrid::from_rows(
        input
            .get_lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Cell::Empty,
                        'S' => Cell::Source,
                        '^' => Cell::Splitter,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect(),
    )
}

#[solution(day = 7, part = 1)]
fn solve_part_1(mut grid: DenseGrid<Cell>) -> usize {
    let source_pos = grid.find(&Cell::Source).next().unwrap();
    let mut pending_beams = vec![source_pos.south()];
    let mut total_splits = 0;

    while let Some(mut pos) = pending_beams.pop() {
        println!("Process beam starting at {pos:?}");

        while let Some(cell) = grid.at(pos) {
            match cell {
                Cell::Empty => {
                    grid.set_at(pos, Cell::Beam);
                    pos = pos.south();
                }
                Cell::Beam => break,
                Cell::Splitter => {
                    total_splits += 1;
                    pending_beams.push(pos.east());
                    pending_beams.push(pos.west());
                    break;
                }
                Cell::Source => unreachable!(),
            }
        }
    }

    total_splits
}

#[solution(day = 7, part = 2)]
fn solve_part_2(grid: DenseGrid<Cell>) -> usize {
    let source_pos = grid.find(&Cell::Source).next().unwrap();
    count_universes(source_pos.south(), &grid, &mut HashMap::new())
}

fn count_universes(
    beam_start: XY,
    grid: &DenseGrid<Cell>,
    cache: &mut HashMap<XY, usize>,
) -> usize {
    let mut pos = beam_start;

    while let Some(cell) = grid.at(pos) {
        match cell {
            Cell::Splitter => {
                // if we've already visited this splitter, then return the cached value
                if let Some(count) = cache.get(&pos) {
                    return *count;
                }

                let count = count_universes(pos.east(), grid, cache)
                    + count_universes(pos.west(), grid, cache);

                cache.insert(pos, count);
                return count;
            }
            Cell::Empty => pos = pos.south(),
            _ => unreachable!(),
        }
    }

    // the beam went out of the board, so there's just one path
    1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_test;

    const TEST_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part_1() {
        aoc_test!(7, 1, 21, TEST_INPUT);
    }

    #[test]
    fn test_part_2() {
        aoc_test!(7, 2, 40, TEST_INPUT);
    }
}
