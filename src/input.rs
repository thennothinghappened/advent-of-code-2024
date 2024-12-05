//! Handling for downloading, caching and generally providing the input data required for each day.

use core::str;
use std::{
    error::Error,
    fmt::{Display, Write as _},
    fs::{create_dir, File},
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

const INPUTS_DIR_PATH: &str = "inputs";
const AOC_COOKIE_PATH: &str = "cookie.txt";

pub(crate) fn load_cookie() -> Result<String, io::Error> {
    File::open(AOC_COOKIE_PATH).map(|mut file| {
        let mut string = String::new();
        file.read_to_string(&mut string).map(|_| string)
    })?
}

pub(crate) fn init_inputs_cache() -> Result<PathBuf, io::Error> {
    let inputs_cache_path = PathBuf::from(INPUTS_DIR_PATH);

    if !inputs_cache_path.exists() {
        create_dir(&inputs_cache_path)?;
    }

    Ok(inputs_cache_path)
}

/// Retrieve either the cached input data for a day, or download the input from AOC and cache it.
pub(crate) fn retrieve_input(
    day: usize,
    cookie_opt: Option<&str>,
    inputs_cache_path: &Path,
) -> Result<String, RetrieveInputError> {
    let input_path = inputs_cache_path.join(format!("day_{}.txt", day));

    if let Ok(mut file) = File::open(&input_path) {
        let mut input = String::new();
        file.read_to_string(&mut input)?;

        return Ok(input);
    }

    let cookie = cookie_opt.ok_or_else(|| RetrieveInputError::NoCookieForDownload)?;

    let input = download_input(day, &cookie)?;
    let mut file = File::create_new(&input_path)?;
    file.write_all(input.as_bytes())?;

    Ok(input)
}

/// Download the solution input for the given day.
fn download_input(day: usize, cookie: &str) -> Result<String, reqwest::Error> {
    println!("Downloading input for day {}...", day);

    let client = reqwest::blocking::Client::new();

    client
        .get(format!("https://adventofcode.com/2024/day/{}/input", day))
        .header(reqwest::header::COOKIE, cookie)
        .send()?
        .text()
}

#[derive(Debug)]
pub(crate) enum RetrieveInputError {
    Io(io::Error),
	Network(reqwest::Error),
    NoCookieForDownload,
}

impl Display for RetrieveInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RetrieveInputError::Io(error) => error.fmt(f),
            RetrieveInputError::NoCookieForDownload => write!(f, "No cookie supplied to download from AOC"),
			RetrieveInputError::Network(error) => error.fmt(f),
        }
    }
}

impl Error for RetrieveInputError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            RetrieveInputError::Io(error) => Some(error),
			RetrieveInputError::Network(error) => Some(error),
            RetrieveInputError::NoCookieForDownload => None,
        }
    }
}

impl From<io::Error> for RetrieveInputError {
    fn from(value: io::Error) -> Self {
        RetrieveInputError::Io(value)
    }
}

impl From<reqwest::Error> for RetrieveInputError {
    fn from(value: reqwest::Error) -> Self {
        RetrieveInputError::Network(value)
    }
}

