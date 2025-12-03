# Advent of Code 2025

An overcomplicated setup for getting inputs and benchmarking the solutions!

# Results

<!---BENCH_START--->

Benchmark CPU: **4x Neoverse-N2**

`|######------------------| 6/24 stars`

| Day                          | Part 1            | Part 2            |
|------------------------------|-------------------|-------------------|
| [01](src/solutions/day01.rs) | 101.4µs / 96 KiB  | 100.6µs / 96 KiB  |
| [02](src/solutions/day02.rs) | 6.0ms / 2 KiB     | 84.4ms / 2 KiB    |
| [03](src/solutions/day03.rs) | 571.0µs / 170 KiB | 999.0µs / 169 KiB |
| 04                           | -                 | -                 |
| 05                           | -                 | -                 |
| 06                           | -                 | -                 |
| 07                           | -                 | -                 |
| 08                           | -                 | -                 |
| 09                           | -                 | -                 |
| 10                           | -                 | -                 |
| 11                           | -                 | -                 |
| 12                           | -                 | -                 |

<!---BENCH_END--->

# Setup

You need to create a file called `token.txt` in the root of this repo with your API key to be able to download
puzzle inputs. You can get this from the session token while logged in on the website.

# Solving

- Create a solution with the format `day{n:02}.rs` in the `src/solutions` folder
- Use the `solution!()` macro to declare solutions. See the template at the end of the readme.
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

The `solution!` macro expands to something like this:

```rust
impl Solver<DAY, 1> for PuzzleInput {
    fn solve(&self) -> Option<impl Display + Debug> {
        Some(solve_part_1(self))
    }
}
```

Then, the build script detects all the solved days and wraps them all into a map of solver functions.

Yes, it's pretty weird, but I'm too far into this rabbit hole to change how it works now ;)
