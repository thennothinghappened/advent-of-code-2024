mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

use core::str;
use std::error::Error;

pub(crate) type PartResult = Result<String, Box<dyn Error>>;
pub(crate) type DayResult = Result<(String, String), Box<dyn Error>>;
pub(crate) type DayFunc = fn(&str) -> DayResult;

pub(crate) const DAYS: &'static [DayFunc] = &[
    day1::solve,
    day2::solve,
    day3::solve,
    day4::solve,
    day5::solve,
    day6::solve,
];
