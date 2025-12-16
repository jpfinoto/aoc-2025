use crate::aoc::*;
use crate::utils::grid::{DIR_DOWN, DIR_LEFT, DIR_RIGHT, DIR_UP, XY};
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
fn solve_part_2(Input(points): Input) -> i64 {
    let edges: Vec<(XY, XY)> = points.iter().cloned().tuple_windows().collect();

    points
        .iter()
        .tuple_combinations()
        .filter_map(|(a, b)| {
            if all_edge_points(*a, *b).all(|p| is_inside(p, &edges)) {
                Some(area(*a, *b))
            } else {
                None
            }
        })
        .max()
        .unwrap()
}

fn area(a: XY, b: XY) -> i64 {
    ((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1)
}

fn all_edge_points(a: XY, b: XY) -> impl Iterator<Item = XY> {
    (a.x..=b.x)
        .map(move |x| XY { x, y: a.y })
        .chain((a.x..=b.x).map(move |x| XY { x, y: b.y }))
        .chain((a.y..=b.y).map(move |y| XY { x: a.x, y }))
        .chain((a.y..=b.y).map(move |y| XY { x: b.x, y }))
}

fn is_inside(point: XY, edges: &[(XY, XY)]) -> bool {
    'outer: for dir in [DIR_RIGHT, DIR_LEFT, DIR_DOWN, DIR_UP] {
        let mut total = 0;
        for intersection in get_ray_intersection(point, dir, edges.iter().cloned()) {
            match intersection {
                RayIntersectionResult::CrossingEdge { .. } => {
                    total += 1;
                }
                RayIntersectionResult::HitCorner(_) => continue 'outer,
            }
        }

        return total % 2 == 1;
    }

    false
}

enum RayIntersectionResult {
    /// Crossing perpendicular to an edge
    CrossingEdge { hit: XY },
    /// The ray hit a corner
    HitCorner(XY),
}

fn get_ray_intersection(
    start: XY,
    direction: XY,
    edges: impl Iterator<Item = (XY, XY)>,
) -> impl Iterator<Item = RayIntersectionResult> {
    edges.flat_map(move |(edge_start, edge_end)| {
        if will_ray_hit(start, direction, edge_start) {
            Some(RayIntersectionResult::HitCorner(edge_start))
        } else if will_ray_hit(start, direction, edge_end) {
            Some(RayIntersectionResult::HitCorner(edge_end))
        } else if let Some(hit) = ray_crosses_segment(start, direction, edge_start, edge_end) {
            Some(RayIntersectionResult::CrossingEdge { hit })
        } else {
            None
        }
    })
}

fn will_ray_hit(start: XY, direction: XY, point: XY) -> bool {
    let (sx, sy) = (start.x, start.y);
    let (px, py) = (point.x, point.y);

    match direction {
        DIR_RIGHT => py == sy && px >= sx,
        DIR_LEFT => py == sy && px <= sx,
        DIR_DOWN => px == sx && py >= sy,
        DIR_UP => px == sx && py <= sy,
        _ => panic!("direction must be cardinal and normalised"),
    }
}

fn ray_crosses_segment(start: XY, direction: XY, edge_start: XY, edge_end: XY) -> Option<XY> {
    let edge_direction = (edge_end - edge_start)
        .normalise_cardinal()
        .expect("edge must be cardinal");

    let potential_hit = match (direction, edge_direction) {
        (DIR_RIGHT | DIR_LEFT, DIR_RIGHT | DIR_LEFT) => None,
        (DIR_UP | DIR_DOWN, DIR_UP | DIR_DOWN) => None,
        (DIR_RIGHT | DIR_LEFT, _)
            if start.x == edge_start.x || (edge_start.x - start.x).signum() == direction.x =>
        {
            let potential_hit = XY {
                x: edge_start.x,
                y: start.y,
            };
            Some(potential_hit)
        }
        (DIR_UP | DIR_DOWN, _)
            if start.y == edge_start.y || (edge_start.y - start.y).signum() == direction.y =>
        {
            let potential_hit = XY {
                x: start.x,
                y: edge_start.y,
            };
            Some(potential_hit)
        }
        _ => None,
    }?;

    if (potential_hit - edge_start).length_sq() <= (edge_end - edge_start).length_sq() {
        Some(potential_hit)
    } else {
        None
    }
}

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
        aoc_test!(9, 2, 24, TEST_INPUT);
    }

    #[test]
    fn test_ray_crosses() {
        assert_eq!(
            ray_crosses_segment((1, 1).into(), DIR_RIGHT, (100, 0).into(), (100, 100).into()),
            Some((100, 1).into())
        );

        assert_eq!(
            ray_crosses_segment((1, 1).into(), DIR_RIGHT, (1, 1).into(), (1, 2).into()),
            Some((1, 1).into())
        );

        assert_eq!(
            ray_crosses_segment((1, 1).into(), DIR_RIGHT, (1, 1).into(), (1, 2).into()),
            Some((1, 1).into())
        );
    }
}
