use crate::BenchmarkMap;
use crate::aoc::{Day, Part};
use crate::bench::BenchmarkResults;
use crate::utils::get_cpu_name;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::env::current_dir;
use std::fs;
use std::path::PathBuf;

pub fn get_cached_benchmarks_path() -> PathBuf {
    current_dir().unwrap().join(".benchmark_cache.toml")
}

pub fn get_cached_benchmarks() -> Option<BenchmarkMap> {
    let cache_file = get_cached_benchmarks_path();
    let cached = fs::read_to_string(&cache_file).ok()?;
    let cached_value: CachedBenchmarks = toml::from_str(&cached).ok()?;

    if cached_value.cpu_name == get_cpu_name() {
        Some(
            cached_value
                .benchmarks
                .into_iter()
                .map(|item| ((item.day, item.part), item.result))
                .collect(),
        )
    } else {
        None
    }
}

pub fn save_cached_benchmarks(cached_benchmarks: &BenchmarkMap) {
    let cache_file = get_cached_benchmarks_path();
    let benchmarks = cached_benchmarks
        .iter()
        .sorted_by_key(|(k, _)| **k)
        .map(|(&(day, part), v)| CachedItem {
            day,
            part,
            result: v.clone(),
        })
        .collect();

    let cached_value = CachedBenchmarks {
        cpu_name: get_cpu_name(),
        benchmarks,
    };
    fs::write(&cache_file, toml::to_string_pretty(&cached_value).unwrap()).unwrap();
}

#[derive(Serialize, Deserialize)]
struct CachedBenchmarks {
    cpu_name: String,
    benchmarks: Vec<CachedItem>,
}

#[derive(Serialize, Deserialize)]
struct CachedItem {
    day: Day,
    part: Part,
    result: BenchmarkResults,
}
