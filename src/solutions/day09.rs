use crate::aoc::*;
use crate::utils::grid::XY;
use derive_solution::{parser, solution};
use itertools::Itertools;

pub struct Input(Vec<XY>);

#[parser]
fn parse_input(input: &PuzzleInput) -> Input {
    Input(
        input
            .get_lines()
            .flat_map(|line| line.try_into().ok())
            .collect(),
    )
}

#[solution(day = 9, part = 1)]
fn solve_part_1(Input(points): Input) -> i64 {
    points
        .iter()
        .tuple_combinations()
        .map(|(a, b)| area(*a, *b))
        .max()
        .unwrap()
}

#[solution(day = 9, part = 2, unsolved)]
#[allow(unused)]
fn solve_part_2(_input: Input) -> i64 {
    0
}

fn area(a: XY, b: XY) -> i64 {
    ((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1)
}

// enum RayIntersectionResult {
//     CrossingEdge {
//         hit: XY,
//         edge_start: XY,
//         edge_end: XY,
//     },
//     OnCorner(XY),
//     AlongEdge {
//         hit: XY,
//         exit: XY,
//     },
// }

// fn get_ray_intersection(
//     start: XY,
//     direction: XY,
//     edges: impl Iterator<Item = (XY, XY)>,
// ) -> RayIntersectionResult {
// }

// fn check_input_data(points: &[XY]) -> bool {
//     // check that there are no points that exist over other edges
//     points.iter().any(|p| )
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_test;

    const TEST_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part_1() {
        aoc_test!(9, 1, 50, TEST_INPUT);
    }

    #[test]
    fn test_part_2() {
        aoc_test!(9, 2, 6, TEST_INPUT);
    }
}
