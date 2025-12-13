# Advent of Code 2025

An overcomplicated setup for getting inputs and benchmarking the solutions!

# Results

<!---BENCH_START--->

Benchmark CPU: **4x Neoverse-N2**

`|######################--| 22/24 stars`

| Day                          | Part 1            | Part 2            |
|------------------------------|-------------------|-------------------|
| [01](src/solutions/day01.rs) | 118.0µs / 96 KiB  | 117.3µs / 96 KiB  |
| [02](src/solutions/day02.rs) | 6.0ms / 2 KiB     | 85.7ms / 2 KiB    |
| [03](src/solutions/day03.rs) | 283.4µs / 26 KiB  | 226.8µs / 26 KiB  |
| [04](src/solutions/day04.rs) | 271.5µs / 59 KiB  | 6.6ms / 82 KiB    |
| [05](src/solutions/day05.rs) | 54.6µs / 16 KiB   | 45.8µs / 16 KiB   |
| [06](src/solutions/day06.rs) | 130.3µs / 142 KiB | 230.4µs / 125 KiB |
| [07](src/solutions/day07.rs) | 77.2µs / 60 KiB   | 212.9µs / 110 KiB |
| [08](src/solutions/day08.rs) | 36.7ms / 15 MiB   | 59.6ms / 15 MiB   |
| [09](src/solutions/day09.rs) | 201.4µs / 12 KiB  | -                 |
| [10](src/solutions/day10.rs) | 1.0ms / 90 KiB    | 11.4ms / 308 KiB  |
| [11](src/solutions/day11.rs) | 155.8µs / 124 KiB | 369.1µs / 233 KiB |
| [12](src/solutions/day12.rs) | 195.0µs / 95 KiB  | -                 |

<!---BENCH_END--->

# Setup

You need to create a file called `token.txt` in the root of this repo with your API key to be able to download
puzzle inputs. You can get this from the session token while logged in on the website.

# Solving

- Create a solution with the format `day{n:02}.rs` in the `src/solutions` folder
- Use the `#[parser]` and `#[solution(day = DAY, part = PART)]` macros to define the input parser and the solution function
- Use `cargo run` to solve the latest solved day
- Alternatively, use `cargo run solve <day>` to solve a specific day.

# Benchmarks

To update the benchmark, run `cargo run --release bench`.

The benchmark runs the solution function for each part of each day, one after the other, and measures the average
execution time and the peak heap usage.

Heap usage is measured on the second call to each solver, so if you have some kind of `lazy_static` that gets allocated
on the first run it will NOT be measured! Stack usage is also not measured.

## GitHub Actions

The benchmark can run automatically via GitHub Actions on every push to main. To enable this:

1. Add your Advent of Code session token as a repository secret named `AOC_TOKEN` (Settings > Secrets and variables > Actions > New repository secret)
2. The workflow will automatically run benchmarks and commit the updated README.md

# Day Template

```rust
use crate::aoc::*;
use derive_solution::{parser, solution};

pub struct Input;

#[parser]
fn parse_input(input: &PuzzleInput) -> Input {
    Input {}
}

#[solution(day = 1, part = 1)]
fn solve_part_1(input: Input) -> i64 {
    0
}

#[solution(day = 1, part = 2)]
fn solve_part_2(input: Input) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_test;

    const TEST_INPUT: &str = "";

    #[test]
    fn test_part_1() {
        aoc_test!(1, 1, 3, TEST_INPUT);
    }

    #[test]
    fn test_part_2() {
        aoc_test!(1, 2, 6, TEST_INPUT);
    }
}
```

# Inner Workings

The `#[parser]` macro generates a `From<&PuzzleInput>` implementation for the input type:

```rust
impl From<&PuzzleInput> for Input {
    fn from(input: &PuzzleInput) -> Self {
        parse_input(input).into()
    }
}
```

The `#[solution]` macro expands to something like this:

```rust
impl Solver<DAY, PART> for PuzzleInput {
    type Input = Input;
    fn solve(&self, input: Self::Input) -> Option<impl Display + Debug> {
        Some(solve_part_1(input))
    }
}
```

Then, the build script detects all the solved days and wraps them all into a map of solver functions.

