use crate::aoc::{Day, PuzzleInput, PuzzleSource, CURRENT_YEAR};
use directories::ProjectDirs;
use reqwest::header::COOKIE;
use sha2::{Digest, Sha256};
use std::fmt::Write;
use std::path::PathBuf;

struct Config {
    year: u32,
    api_token: String,
}

pub struct CachedOnlinePuzzleSource {
    cache_directory: PathBuf,
    config: Config,
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().fold(String::new(), |mut output, b| {
        let _ = write!(output, "{b:02x}");
        output
    })
}

impl CachedOnlinePuzzleSource {
    fn new_with_default_directory(config: Config) -> Result<Self, OnlinePuzzleSourceCreateError> {
        let project_dirs = ProjectDirs::from("", "", env!("CARGO_PKG_NAME")).unwrap();

        let token_hash = hex_encode(&Sha256::digest(config.api_token.as_bytes())[..10]);

        let cache_directory = project_dirs
            .cache_dir()
            .to_path_buf()
            .join(&token_hash)
            .join("inputs")
            .join(config.year.to_string());

        std::fs::create_dir_all(&cache_directory).map_err(|e| {
            OnlinePuzzleSourceCreateError::FailedToCreateCacheDirectory {
                path: cache_directory.clone(),
                message: e.to_string(),
            }
        })?;

        Ok(CachedOnlinePuzzleSource {
            config,
            cache_directory,
        })
    }

    pub fn new() -> Result<Self, OnlinePuzzleSourceCreateError> {
        let path = std::env::current_dir()?.join("token.txt");

        let api_token = std::fs::read_to_string(path.clone()).map_err(|e| {
            OnlinePuzzleSourceCreateError::FailedToLoadToken {
                path,
                message: e.to_string(),
            }
        })?;

        Self::new_with_default_directory(Config {
            year: CURRENT_YEAR,
            api_token,
        })
    }

    fn get_day_path(&self, day: Day) -> PathBuf {
        self.cache_directory
            .join(day.to_string())
            .with_extension("txt")
    }

    fn get_input_from_api(&self, day: Day) -> Result<String, PuzzleInputApiError> {
        let url = format!(
            "https://adventofcode.com/{}/day/{}/input",
            self.config.year, day
        );
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(&url)
            .header(COOKIE, format!("session={}", self.config.api_token))
            .send()
            .map_err(|e| PuzzleInputApiError::ApiError(e.to_string()))?;

        let data = response.text().unwrap();

        if data.starts_with(DAY_NOT_UNLOCKED_START) {
            Err(PuzzleInputApiError::NotUnlocked)
        } else {
            Ok(data)
        }
    }

    fn download_and_cache(&self, day: Day) -> Result<String, PuzzleInputSaveError> {
        let data = self.get_input_from_api(day)?;

        std::fs::write(self.get_day_path(day), data.clone())?;

        Ok(data)
    }
}

impl PuzzleSource for CachedOnlinePuzzleSource {
    fn get_input(&self, day: Day) -> Result<PuzzleInput, Box<dyn std::error::Error>> {
        let path = self.get_day_path(day);

        if let Ok(contents) = std::fs::read_to_string(path.clone()) {
            log::trace!(
                "reading cached input for day {day} at {}",
                path.to_str().unwrap()
            );
            return Ok(contents.as_str().into());
        }

        log::debug!(
            "downloading input for day {day} to {}",
            path.to_str().unwrap()
        );

        let contents = self.download_and_cache(day)?;

        Ok(contents.as_str().into())
    }
}

const DAY_NOT_UNLOCKED_START: &str =
    "Please don't repeatedly request this endpoint before it unlocks!";

#[derive(thiserror::Error, Debug)]
pub enum OnlinePuzzleSourceCreateError {
    #[error("failed to load token at {path}: {message}")]
    FailedToLoadToken { path: PathBuf, message: String },
    #[error("failed to create cache directory {path}: {message}")]
    FailedToCreateCacheDirectory { path: PathBuf, message: String },
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum PuzzleInputApiError {
    #[error("Advent of Code API returned an error: {0}")]
    ApiError(String),
    #[error("Puzzle not unlocked")]
    NotUnlocked,
}

#[derive(thiserror::Error, Debug)]
pub enum PuzzleInputSaveError {
    #[error("failed to download puzzle input: {0}")]
    ApiError(#[from] PuzzleInputApiError),
    #[error("failed to save puzzle input: {0}")]
    IoError(#[from] std::io::Error),
}
