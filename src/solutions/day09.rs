use crate::aoc::*;
#[cfg(test)]
use crate::utils::grid::DenseGrid;
use crate::utils::grid::{DIR_DOWN, DIR_LEFT, DIR_RIGHT, DIR_UP, XY};
use derive_solution::{parser, solution};
use itertools::Itertools;
use rayon::prelude::*;
use std::iter;

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

#[solution(day = 9, part = 2)]
#[allow(unused)]
fn solve_part_2(Input(points): Input) -> i64 {
    let edges: Vec<(XY, XY)> = points
        .iter()
        .chain(iter::once(points.first().unwrap()))
        .cloned()
        .tuple_windows()
        .collect();

    #[cfg(test)]
    {
        let size = points.iter().map(|p| p.x.max(p.y)).max().unwrap() + 3;

        let grid = DenseGrid::from_iter(
            (size + 1) as usize,
            (0..=size).cartesian_product(0..=size).map(|(x, y)| {
                let point: XY = (y, x).into();

                if points.contains(&point) {
                    "#"
                } else if is_inside(point, &edges) {
                    "X"
                } else {
                    "."
                }
            }),
        );

        println!("{grid}");
    }

    let potential_sorted = points
        .iter()
        .tuple_combinations()
        .sorted_by_key(|&(a, b)| -area(*a, *b))
        .collect_vec();

    let total = potential_sorted.len();

    potential_sorted
        .par_iter()
        .cloned()
        .enumerate()
        .find_map_first(|(i, (a, b))| {
            let area = area(*a, *b);

            if points.iter().any(|p| is_point_inside_rectangle(*p, *a, *b)) {
                None
            } else if all_edge_points(*a, *b).all(|p| is_inside(p, &edges)) {
                Some(area)
            } else {
                None
            }
        })
        .expect("no solution found")
}

fn area(a: XY, b: XY) -> i64 {
    ((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1)
}

fn is_point_inside_rectangle(point: XY, a: XY, b: XY) -> bool {
    ((a.x.min(b.x) + 1)..a.x.max(b.x)).contains(&point.x)
        && ((a.y.min(b.y) + 1)..a.y.max(b.y)).contains(&point.y)
}

fn all_edge_points(a: XY, b: XY) -> impl Iterator<Item = XY> {
    let from_x = a.x.min(b.x);
    let from_y = a.y.min(b.y);
    let to_x = a.x.max(b.x);
    let to_y = a.y.max(b.y);

    (from_x..=to_x)
        .map(move |x| XY { x, y: a.y })
        .chain((from_x..=to_x).map(move |x| XY { x, y: b.y }))
        .chain((from_y..=to_y).map(move |y| XY { x: a.x, y }))
        .chain((from_y..=to_y).map(move |y| XY { x: b.x, y }))
}

fn is_inside(point: XY, edges: &[(XY, XY)]) -> bool {
    'outer: for dir in [DIR_LEFT, DIR_RIGHT, DIR_DOWN, DIR_UP] {
        let mut total = 0;
        for intersection in get_ray_intersection(point, dir, edges.iter().cloned()) {
            match intersection {
                RayIntersectionResult::InsideEdge => return true,
                RayIntersectionResult::CrossingEdge => {
                    total += 1;
                }
                RayIntersectionResult::HitCorner => continue 'outer,
            }
        }

        return total % 2 == 1;
    }

    panic!("no direction gave a good ray-cast for {point}")
}

enum RayIntersectionResult {
    /// Crossing perpendicular to an edge
    CrossingEdge,
    /// The ray hit a corner
    HitCorner,
    /// The starting point is inside an edge
    InsideEdge,
}

fn get_ray_intersection(
    start: XY,
    direction: XY,
    edges: impl Iterator<Item = (XY, XY)>,
) -> impl Iterator<Item = RayIntersectionResult> {
    edges.flat_map(move |(edge_start, edge_end)| {
        if is_inside_edge(start, edge_start, edge_end) {
            Some(RayIntersectionResult::InsideEdge)
        } else if will_ray_hit(start, direction, edge_start)
            || will_ray_hit(start, direction, edge_end)
        {
            Some(RayIntersectionResult::HitCorner)
        } else {
            ray_crosses_segment(start, direction, edge_start, edge_end)
                .map(|_| RayIntersectionResult::CrossingEdge)
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

    if is_inside_edge(potential_hit, edge_start, edge_end) {
        Some(potential_hit)
    } else {
        None
    }
}

fn is_inside_edge(point: XY, edge_start: XY, edge_end: XY) -> bool {
    let x_range = edge_start.x.min(edge_end.x)..=edge_start.x.max(edge_end.x);
    let y_range = edge_start.y.min(edge_end.y)..=edge_start.y.max(edge_end.y);

    x_range.contains(&point.x) && y_range.contains(&point.y)
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
