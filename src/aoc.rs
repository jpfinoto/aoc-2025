use std::collections::HashMap;
use std::fmt::{Debug, Display};

pub type Day = usize;
pub type Part = usize;
pub type SolverMap = HashMap<(Day, Part), Box<dyn Fn(&PuzzleInput) -> Option<String>>>;

pub const CURRENT_YEAR: u32 = 2025;

#[allow(clippy::test_attr_in_doctest)]
/// Declare a test to run a part
///
/// Usage:
/// ```rust
/// fn part_1(input: impl Lines) -> u64 {
///     // implementation
///     3159281
/// }
///
/// solution!(24, part_1)
///
/// #[test]
/// fn test_part_1() {
///     aoc_test!(24, 1, 3159281, "8172638174891\n19294378171");
/// }
/// ```
#[macro_export]
macro_rules! aoc_test {
    ($day:expr, $part:literal, $expected:expr, $content:expr) => {
        let puzzle_input: PuzzleInput = $content.into();
        let result =
            <PuzzleInput as Solver<$day, $part>>::solve(&puzzle_input, (&puzzle_input).into())
                .expect("no result")
                .to_string();
        assert_eq!(result, $expected.to_string());
    };
}

pub struct PuzzleInput {
    input: String,
}

impl PuzzleInput {
    pub fn get_raw(&self) -> &str {
        &self.input
    }

    pub fn get_lines(&self) -> impl Iterator<Item = &str> {
        self.get_raw().lines().map(str::trim)
    }
}

impl<'a> From<&'a str> for PuzzleInput {
    fn from(value: &'a str) -> Self {
        Self {
            input: value.to_owned(),
        }
    }
}

impl From<&Vec<String>> for PuzzleInput {
    fn from(value: &Vec<String>) -> Self {
        Self {
            input: value.join("\n"),
        }
    }
}

pub trait Solver<const D: usize, const P: usize> {
    type Input: for<'a> From<&'a PuzzleInput>;
    fn solve(&self, input: Self::Input) -> Option<impl Display + Debug>;
}

pub trait PuzzleSource {
    fn get_input(&self, day: Day) -> anyhow::Result<PuzzleInput>;
}

pub struct FixedDataSource {
    pub lines: Vec<String>,
}

impl PuzzleSource for FixedDataSource {
    fn get_input(&self, _day: Day) -> anyhow::Result<PuzzleInput> {
        Ok(PuzzleInput::from(&self.lines))
    }
}

pub fn get_days_iter() -> impl Iterator<Item = Day> {
    1..=12
}

#[cfg(test)]
mod tests {
    use super::*;
    use derive_solution::{parser, solution};

    pub struct Input(Vec<u64>);

    #[solution(day = 100, part = 1)]
    fn sum_lines(Input(input): Input) -> u64 {
        input.into_iter().sum()
    }

    #[parser]
    fn parse(input: &PuzzleInput) -> Input {
        Input(
            input
                .get_lines()
                .map(|l| l.parse::<u64>().unwrap())
                .collect(),
        )
    }

    impl From<PuzzleInput> for Vec<u64> {
        fn from(input: PuzzleInput) -> Self {
            input
                .get_lines()
                .map(|l| l.parse::<u64>().unwrap())
                .collect()
        }
    }

    #[test]
    fn test_implemented_solver() {
        aoc_test!(100, 1, 6, "1\n2\n3");
    }
}
