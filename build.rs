use quote::quote;
use regex::Regex;
use std::error::Error;
use std::path::Path;

fn get_solved_days(solutions_path: &Path) -> Result<Vec<usize>, Box<dyn Error>> {
    let day_regex = Regex::new(r"^day(?<day>\d{2})\.rs$").unwrap();
    std::fs::read_dir(solutions_path)?
        .filter_map(|entry_result| {
            let entry = match entry_result.map_err(|e| format!("Failed to read entry: {}", e)) {
                Ok(entry) => entry,
                Err(e) => {
                    return Some(Err(e));
                }
            };
            let file_name_raw = entry.file_name();
            let file_name = file_name_raw.to_string_lossy();
            // if this file name doesn't match the regex, skip it
            let captures = day_regex.captures(&file_name)?;
            Some(Ok(captures
                .name("day")
                .expect("the regex should always capture the day number")
                .as_str()
                .parse::<usize>()
                .expect(
                    "the regex should always capture a valid day number",
                )))
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(From::from)
}

fn main() {
    println!("cargo::rerun-if-changed=src/solutions");
    println!("cargo::rerun-if-changed=build.rs");

    let solutions_path = Path::new("src/solutions");
    let solved_days = get_solved_days(solutions_path).unwrap();

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("solved_days.rs");

    let generated_code = generate_solver_map(solved_days);

    std::fs::write(&dest_path, generated_code).expect("Failed to write solved_days.rs");
}

fn generate_solver_map(days: Vec<usize>) -> String {
    let hashmap_code = quote! {
        use std::collections::HashMap;
        use crate::aoc::{PuzzleInput, Solver, SolverMap};

        pub fn get_solvers() -> SolverMap {
            let mut map: SolverMap = HashMap::new();
            #(map.insert(
                (#days, 1),
                Box::new(
                    |input| <PuzzleInput as Solver<#days, 1>>::solve(input).map(|solution| solution.to_string())
                )
            );)*
            #(map.insert(
                (#days, 2),
                Box::new(
                    |input| <PuzzleInput as Solver<#days, 2>>::solve(input).map(|solution| solution.to_string())
                )
            );)*
            map
        }
    };

    // Format the generated code with prettyplease.
    prettyplease::unparse(&syn::parse2(hashmap_code).unwrap())
}
