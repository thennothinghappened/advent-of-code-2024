
mod utils;
mod day1;
mod day2;
mod day3;
mod day4;

use core::str;
use std::error::Error;

pub type DayFunc = fn(&str) -> Result<(String, String), Box<dyn Error>>;
pub const DAYS: &'static [DayFunc] = &[day1::solve, day2::solve, day3::solve, day4::solve];
