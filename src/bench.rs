use crate::PEAK_ALLOC;
use serde::{Deserialize, Serialize};
use std::cmp::min;
use std::fmt::{Display, Formatter};
use std::time::{Duration, Instant};

const TARGET_DURATION_PER_PART: Duration = Duration::from_secs(5);
const MAX_RUNS: usize = 1_000_000_000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct BenchmarkResults {
    pub iterations: usize,
    pub average_duration: Duration,
    pub peak_memory: usize,
}

pub fn format_duration(duration: Duration) -> String {
    if duration < Duration::from_micros(1) {
        format!("{}ns", duration.subsec_nanos())
    } else if duration < Duration::from_millis(1) {
        format!("{:.1}µs", duration.as_secs_f64() * 1_000_000.0)
    } else if duration < Duration::from_secs(1) {
        format!("{:.1}ms", duration.as_secs_f64() * 1_000.0)
    } else {
        format!("{:.1}s", duration.as_secs_f64())
    }
}

const ONE_KB: usize = 1024;
const ONE_MB: usize = ONE_KB * 1024;

pub fn format_memory(bytes: usize) -> String {
    if bytes < ONE_KB {
        format!("{bytes} bytes")
    } else if bytes < ONE_MB {
        format!("{} KiB", bytes / ONE_KB)
    } else {
        format!("{} MiB", bytes / ONE_MB)
    }
}

impl Display for BenchmarkResults {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let iter = match self.iterations {
            1 => "1 iteration".to_string(),
            n => format!("{} iterations", n),
        };

        write!(
            f,
            "{} / {} peak ({iter})",
            format_duration(self.average_duration),
            format_memory(self.peak_memory),
        )
    }
}

pub(crate) fn benchmark<T, F: Fn() -> Option<T>>(
    bench_fn: F,
) -> Result<BenchmarkResults, BenchmarkError> {
    // run the function to get an idea of how long it takes
    let start = Instant::now();
    let _ = bench_fn().ok_or(BenchmarkError::NotImplemented)?;
    let first_run_duration = start.elapsed();

    // measure the memory usage
    // it's important that this is done in a second run because the stdlib might allocate
    // things when first called, which would mess up the memory usage for part 1
    PEAK_ALLOC.reset_peak_usage();
    let initial_mem = PEAK_ALLOC.current_usage();

    bench_fn();

    let peak_mem = PEAK_ALLOC.peak_usage();
    let used_mem = peak_mem - initial_mem;

    if first_run_duration > TARGET_DURATION_PER_PART {
        Ok(BenchmarkResults {
            iterations: 1,
            average_duration: first_run_duration,
            peak_memory: used_mem,
        })
    } else {
        let project_runs = (TARGET_DURATION_PER_PART.as_secs_f64()
            / first_run_duration.as_secs_f64())
        .ceil() as usize;
        let iterations = min(MAX_RUNS, project_runs);

        let start = Instant::now();
        for _ in 0..iterations {
            _ = bench_fn().unwrap();
        }
        let duration = start.elapsed();

        Ok(BenchmarkResults {
            iterations,
            average_duration: duration / (iterations as u32),
            peak_memory: used_mem,
        })
    }
}

#[derive(Debug, thiserror::Error, Eq, PartialEq)]
pub(crate) enum BenchmarkError {
    #[error("not implemented")]
    NotImplemented,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    fn very_fast_solver() -> Option<()> {
        sleep(Duration::from_millis(10));
        Some(())
    }

    fn very_slow_solver() -> Option<()> {
        sleep(Duration::from_millis(2000));
        Some(())
    }

    fn not_implemented_solver() -> Option<()> {
        None
    }

    #[allow(clippy::useless_vec)]
    fn alloc_vec_solver() -> Option<i64> {
        let vec = vec![1, 2, 3, 4];
        Some(vec.iter().sum())
    }

    fn factorial_stack(n: f64) -> f64 {
        if n > 1.0 {
            n * factorial_stack(n - 1.0)
        } else {
            1.0
        }
    }

    #[test]
    fn test_benchmark_slow_solver() {
        let bench = benchmark(very_slow_solver);
        assert!(bench.is_ok());
        assert_eq!(1, bench.unwrap().iterations);
    }

    #[test]
    fn test_benchmark_fast_solver() {
        let bench = benchmark(very_fast_solver);
        assert!(bench.is_ok());
        assert!(bench.unwrap().iterations > 10);
    }

    #[test]
    fn test_benchmark_fails() {
        let bench = benchmark(not_implemented_solver);
        assert_eq!(Some(BenchmarkError::NotImplemented), bench.err());
    }

    #[ignore]
    #[test]
    fn test_benchmark_alloc_vec_solver() {
        let bench = benchmark(alloc_vec_solver);
        assert!(bench.is_ok());
        assert!(bench.unwrap().peak_memory >= 4 * 4); // 4 * i64 numbers
    }

    #[ignore]
    #[test]
    fn test_benchmark_stack_memory_solver() {
        let bench = benchmark(|| Some(factorial_stack(1000.0)));
        assert!(bench.is_ok());
        assert_eq!(bench.unwrap().peak_memory, 0);
    }
}
