pub mod aoc;
pub mod bench;
mod benchmark_cache;
pub mod inputs;
mod readme;
pub mod solutions;
mod utils;

use crate::aoc::{Day, Part, PuzzleSource, SolverMap, get_days_iter};
use crate::bench::{BenchmarkResults, benchmark};
use crate::benchmark_cache::{get_cached_benchmarks, save_cached_benchmarks};
use crate::inputs::CachedOnlinePuzzleSource;
use crate::readme::update_readme;
use crate::solutions::get_solvers;
use clap::{Command, arg, command};
use peak_alloc::PeakAlloc;
use std::cell::LazyCell;
use std::collections::HashMap;
use std::iter;

#[global_allocator]
pub static PEAK_ALLOC: PeakAlloc = PeakAlloc;

type BenchmarkMap = HashMap<(Day, Part), BenchmarkResults>;

fn main() -> Result<(), String> {
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let puzzle_source = CachedOnlinePuzzleSource::new().expect("failed to configure puzzle source");
    let solvers = get_solvers();

    let matches = command!()
        .subcommand(
            Command::new("bench")
                .about("Run the benchmark")
                .arg(arg!([day] "which day to run")),
        )
        .subcommand(
            Command::new("solve")
                .about("Solve a day")
                .arg(arg!([day] "which day to solve")),
        )
        .get_matches();

    if let Some(bench_args) = matches.subcommand_matches("bench") {
        let benchmarks = {
            if let Some(day) = bench_args.get_one::<String>("day") {
                get_cached_benchmarks()
                    .map(|mut bench| {
                        bench.extend(run_benchmarks(
                            &solvers,
                            &puzzle_source,
                            iter::once(day.parse::<Day>().unwrap()),
                        ));
                        bench
                    })
                    .or_else(|| Some(run_benchmarks(&solvers, &puzzle_source, get_days_iter())))
                    .unwrap()
            } else {
                run_benchmarks(&solvers, &puzzle_source, get_days_iter())
            }
        };
        save_cached_benchmarks(&benchmarks);
        update_readme(&benchmarks);
        Ok(())
    } else if let Some(solve_args) = matches.subcommand_matches("solve") {
        if let Some(day) = solve_args.get_one::<String>("day") {
            solve_one(&solvers, &puzzle_source, day.parse::<Day>().unwrap())
        } else {
            solve_latest(&solvers, &puzzle_source)
        }
    } else {
        solve_latest(&solvers, &puzzle_source)
    }
}

fn solve_latest(solvers: &SolverMap, puzzle_source: &impl PuzzleSource) -> Result<(), String> {
    solve_one(
        solvers,
        puzzle_source,
        get_last_day(solvers).ok_or("no solved days".to_string())?,
    )
}

fn get_last_day(solver_map: &SolverMap) -> Option<Day> {
    get_days_iter()
        .flat_map(|day| {
            solver_map
                .get(&(day, 1))
                .or(solver_map.get(&(day, 2)))
                .and(Some(day))
        })
        .last()
}

fn solve_one(
    solver_map: &SolverMap,
    puzzle_source: &impl PuzzleSource,
    day: Day,
) -> Result<(), String> {
    let input = puzzle_source
        .get_input(day)
        .expect("failed to get puzzle input");
    println!("Day {day}");
    println!(
        "- part 1: {}",
        solver_map
            .get(&(day, 1))
            .and_then(|solver| solver(&input))
            .unwrap_or("-".to_string()),
    );
    println!(
        "- part 2: {}",
        solver_map
            .get(&(day, 2))
            .and_then(|solver| solver(&input))
            .unwrap_or("-".to_string()),
    );

    Ok(())
}

fn run_benchmarks(
    solver_map: &SolverMap,
    puzzle_source: &impl PuzzleSource,
    days: impl Iterator<Item = Day>,
) -> BenchmarkMap {
    let mut all_results = HashMap::new();

    for day in days {
        let mut part_bench: BenchmarkMap = HashMap::new();
        let input = LazyCell::new(|| {
            puzzle_source
                .get_input(day)
                .expect("failed to get puzzle input")
        });
        for part in 1..=2 as Part {
            if let Some(solver) = solver_map.get(&(day, part)) {
                let bench = benchmark(|| solver(&input));
                if let Ok(result) = bench {
                    part_bench.insert((day, part), result);
                } else {
                    log::debug!("day {day} part {part} not solved");
                }
            }
        }

        all_results.extend(part_bench.clone());

        if !part_bench.is_empty() {
            log::info!(
                "Day {day}: \n - part 1: {}\n - part 2: {} ",
                part_bench
                    .get(&(day, 1))
                    .map(|t| t.to_string())
                    .unwrap_or("-".to_string()),
                part_bench
                    .get(&(day, 2))
                    .map(|t| t.to_string())
                    .unwrap_or("-".to_string()),
            );
        }
    }

    all_results
}
