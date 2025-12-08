use crate::aoc::*;
use derive_solution::{parser, solution};
use itertools::Itertools;
use log::debug;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    pub fn distance2(&self, other: &Point) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }

    pub fn from_str(s: &str) -> Option<Point> {
        let [x, y, z] = s
            .split(',')
            .flat_map(|s| s.parse::<i64>())
            .collect_vec()
            .try_into()
            .ok()?;

        Some(Point { x, y, z })
    }
}

#[parser]
fn parse_input(input: &PuzzleInput) -> Vec<Point> {
    input.get_lines().filter_map(Point::from_str).collect_vec()
}

#[solution(day = 8, part = 1)]
fn solve_part_1(input: Vec<Point>) -> usize {
    let total_connections = if cfg!(test) { 10 } else { 1000 };

    let (circuit_members, _) = connect_until(input, Some(total_connections));

    circuit_members
        .values()
        .map(|list| list.len())
        .sorted_by_key(|&n| -(n as i32))
        .take(3)
        .product()
}

#[solution(day = 8, part = 2)]
fn solve_part_2(input: Vec<Point>) -> i64 {
    let (_, (a, b)) = connect_until(input, None);

    a.x * b.x
}

fn connect_until(
    input: Vec<Point>,
    total_connections: Option<usize>,
) -> (HashMap<usize, HashSet<Point>>, (Point, Point)) {
    let mut circuit_id = 0;
    let mut junction_circuit: HashMap<Point, usize> = HashMap::new();
    let mut circuit_members: HashMap<usize, HashSet<Point>> = HashMap::new();

    let mut last_connection = None;

    let pairs = input
        .iter()
        .tuple_combinations()
        .sorted_by_key(|&(a, b)| a.distance2(b))
        .collect_vec();

    for (a, b) in pairs
        .into_iter()
        .enumerate()
        .take_while(|(i, _)| total_connections.is_none() || i < &total_connections.unwrap())
        .map(|(_, (a, b))| (a, b))
    {
        debug!("=> Checking {a:?} {b:?}");
        match (
            junction_circuit.get(a).cloned(),
            junction_circuit.get(b).cloned(),
        ) {
            (Some(circuit_a), Some(circuit_b)) if circuit_a != circuit_b => {
                debug!(
                    "Circuit {circuit_a:?} and {circuit_b:?} are not the same, reassigning points"
                );
                last_connection = Some((*a, *b));
                reassign_points(
                    circuit_b,
                    circuit_a,
                    &mut junction_circuit,
                    &mut circuit_members,
                );
            }
            (None, Some(circuit)) => {
                debug!("Join a on {circuit:?} ");
                last_connection = Some((*a, *b));
                junction_circuit.insert(*a, circuit);
                circuit_members.entry(circuit).or_default().insert(*a);
            }
            (Some(circuit), None) => {
                debug!("Join b on {circuit:?} ");
                last_connection = Some((*a, *b));
                junction_circuit.insert(*b, circuit);
                circuit_members.entry(circuit).or_default().insert(*b);
            }
            (None, None) => {
                debug!("Creating new circuit for {a:?} {b:?} => {circuit_id}");
                last_connection = Some((*a, *b));
                junction_circuit.insert(*a, circuit_id);
                junction_circuit.insert(*b, circuit_id);
                circuit_members
                    .entry(circuit_id)
                    .or_default()
                    .extend(vec![*a, *b].into_iter());
                circuit_id += 1;
            }
            (Some(a), Some(b)) if a == b => {
                debug!("a and b are already in the same circuit {a}");
            }
            _ => unreachable!(),
        }
    }

    (circuit_members, last_connection.unwrap())
}

fn reassign_points(
    old_circuit: usize,
    new_circuit: usize,
    junction_circuit: &mut HashMap<Point, usize>,
    circuit_members: &mut HashMap<usize, HashSet<Point>>,
) {
    let members_to_reassign = circuit_members
        .get(&old_circuit)
        .unwrap()
        .iter()
        .cloned()
        .collect_vec();

    circuit_members.remove(&old_circuit);

    for point in members_to_reassign {
        if let Some(old_circuit) = junction_circuit.get(&point)
            && let Some(members) = circuit_members.get_mut(old_circuit)
        {
            members.remove(&point);
        }
        junction_circuit.insert(point, new_circuit);
        circuit_members
            .entry(new_circuit)
            .or_default()
            .insert(point);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_test;

    const TEST_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_part_1() {
        aoc_test!(8, 1, 40, TEST_INPUT);
    }

    #[test]
    fn test_part_2() {
        aoc_test!(8, 2, 25272, TEST_INPUT);
    }
}
