use std::collections::HashMap;
use std::fmt::{Debug, Display};

pub type Day = usize;
pub type Part = usize;
pub type SolverMap = HashMap<(Day, Part), Box<dyn Fn(&PuzzleInput) -> Option<String>>>;

pub const CURRENT_YEAR: u32 = 2025;

#[macro_export]
macro_rules! solution {
    ($day:expr) => {
        use std::fmt::{Debug, Display};
        use $crate::aoc::{Lines, PuzzleInput};

        impl Solver<$day, 1> for PuzzleInput {
            fn solve(&self) -> Option<impl Display + Debug> {
                None as Option<String>
            }
        }
        impl Solver<$day, 2> for PuzzleInput {
            fn solve(&self) -> Option<impl Display + Debug> {
                None as Option<String>
            }
        }
    };
    ($day:expr, $part_1_solver:ident) => {
        use std::fmt::{Debug, Display};

        impl Solver<$day, 1> for PuzzleInput {
            fn solve(&self) -> Option<impl Display + Debug> {
                Some($part_1_solver(self))
            }
        }
        impl Solver<$day, 2> for PuzzleInput {
            fn solve(&self) -> Option<impl Display + Debug> {
                None as Option<String>
            }
        }
    };
    ($day:expr, $part_1_solver:ident, $part_2_solver:ident) => {
        use std::fmt::{Debug, Display};

        impl Solver<$day, 1> for PuzzleInput {
            fn solve(&self) -> Option<impl Display + Debug> {
                Some($part_1_solver(self))
            }
        }
        impl Solver<$day, 2> for PuzzleInput {
            fn solve(&self) -> Option<impl Display + Debug> {
                Some($part_2_solver(self))
            }
        }
    };
}

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
        let input: PuzzleInput = $content.into();
        let result = <PuzzleInput as Solver<$day, $part>>::solve(&input)
            .expect("no result")
            .to_string();
        assert_eq!(result, $expected.to_string());
    };
}

pub struct PuzzleInput {
    input: String,
}

pub trait Lines {
    fn get_raw(&self) -> &str;
    fn get_lines(&self) -> impl Iterator<Item = &str> {
        self.get_raw().lines().map(|s| s.trim())
    }
}

impl Lines for PuzzleInput {
    fn get_raw(&self) -> &str {
        self.input.as_str()
    }
}

impl Lines for &PuzzleInput {
    fn get_raw(&self) -> &str {
        self.input.as_str()
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
    fn solve(&self) -> Option<impl Display + Debug>;
}

pub trait PuzzleSource {
    fn get_input(&self, day: Day) -> Result<PuzzleInput, Box<dyn std::error::Error>>;
}

pub struct FixedDataSource {
    pub lines: Vec<String>,
}

impl PuzzleSource for FixedDataSource {
    fn get_input(&self, _day: Day) -> Result<PuzzleInput, Box<dyn std::error::Error>> {
        Ok(PuzzleInput::from(&self.lines))
    }
}

pub fn get_days_iter() -> impl Iterator<Item = Day> {
    1..=12
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sum_lines(input: impl Lines) -> u64 {
        input.get_lines().map(|l| l.parse::<u64>().unwrap()).sum()
    }

    solution!(100, sum_lines);

    #[test]
    fn test_implemented_solver() {
        aoc_test!(100, 1, 6, "1\n2\n3");
    }
}
