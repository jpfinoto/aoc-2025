use crate::aoc::*;
use derive_solution::{parser, solution};
use smol_str::SmolStr;
use std::collections::HashMap;

pub struct Network {
    connections: HashMap<SmolStr, Vec<SmolStr>>,
}

#[parser]
fn parse_input(input: &PuzzleInput) -> Network {
    let connections = input
        .get_lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (source, targets) = line.split_once(':').unwrap();
            (
                SmolStr::new(source),
                targets
                    .split(' ')
                    .map(|target| target.trim())
                    .filter(|target| !target.is_empty())
                    .map(SmolStr::new)
                    .collect(),
            )
        })
        .collect();

    Network { connections }
}

#[solution(day = 11, part = 1)]
fn solve_part_1(network: Network) -> i64 {
    network.count_paths("you", "out")
}

#[solution(day = 11, part = 2)]
fn solve_part_2(network: Network) -> i64 {
    network.count_paths_going_through_fft_and_dac(
        State {
            current: SmolStr::from("svr"),
            passed_through_fft: false,
            passed_through_dac: false,
        },
        SmolStr::from("out"),
        &mut HashMap::new(),
    )
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    current: SmolStr,
    passed_through_fft: bool,
    passed_through_dac: bool,
}

impl Network {
    fn count_paths(&self, from: &str, to: &str) -> i64 {
        self.count_paths_inner(SmolStr::from(from), SmolStr::from(to), &mut HashMap::new())
    }

    fn count_paths_inner(
        &self,
        from: SmolStr,
        to: SmolStr,
        cache: &mut HashMap<SmolStr, i64>,
    ) -> i64 {
        if from == to {
            return 1;
        }

        if let Some(cached_value) = cache.get(&from) {
            return *cached_value;
        }

        let paths = self.connections[&from]
            .iter()
            .map(|target| self.count_paths_inner(target.clone(), to.clone(), cache))
            .sum();

        cache.insert(from, paths);

        paths
    }

    fn count_paths_going_through_fft_and_dac(
        &self,
        mut state: State,
        to: SmolStr,
        cache: &mut HashMap<State, i64>,
    ) -> i64 {
        if let Some(cached_value) = cache.get(&state) {
            return *cached_value;
        }

        if state.current == to {
            return if state.passed_through_dac && state.passed_through_fft {
                1
            } else {
                0
            };
        }

        if state.current == "fft" {
            state.passed_through_fft = true;
        } else if state.current == "dac" {
            state.passed_through_dac = true;
        }

        let paths = self.connections[&state.current]
            .iter()
            .map(|next| {
                self.count_paths_going_through_fft_and_dac(
                    State {
                        current: next.clone(),
                        ..state
                    },
                    to.clone(),
                    cache,
                )
            })
            .sum();

        cache.insert(state, paths);

        paths
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_test;

    const TEST_INPUT: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    const TEST_INPUT2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_part_1() {
        aoc_test!(11, 1, 5, TEST_INPUT);
    }

    #[test]
    fn test_part_2() {
        aoc_test!(11, 2, 2, TEST_INPUT2);
    }
}
