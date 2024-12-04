use core::str;
use curl::easy::Easy;
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
fn download_input(day: usize, cookie: &str) -> Result<String, io::Error> {
    println!("Downloading input for day {}...", day);

    let mut input = String::new();
    let mut request = Easy::new();

    request.url(&format!("https://adventofcode.com/2024/day/{}/input", day))?;
    request.cookie(cookie)?;

    {
        let mut transfer = request.transfer();

        transfer.write_function(|data| {
            input
                .write_str(str::from_utf8(data).unwrap())
                .map(|_| Ok(input.len()))
                .expect("Very bad things have happened while writing input to a string!!!")
        })?;

        transfer.perform()?;
    }

    Ok(input)
}

#[derive(Debug)]
pub(crate) enum RetrieveInputError {
    Io(io::Error),
    NoCookieForDownload,
}

impl Display for RetrieveInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RetrieveInputError::Io(error) => error.fmt(f),
            RetrieveInputError::NoCookieForDownload => {
                write!(f, "No cookie supplied to download from AOC")
            }
        }
    }
}

impl Error for RetrieveInputError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            RetrieveInputError::Io(error) => Some(error),
            RetrieveInputError::NoCookieForDownload => None,
        }
    }
}

impl From<io::Error> for RetrieveInputError {
    fn from(value: io::Error) -> Self {
        RetrieveInputError::Io(value)
    }
}
