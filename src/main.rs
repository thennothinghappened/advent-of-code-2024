mod day1;
mod day2;
mod input;

use core::str;
use std::error::Error;

type DayFunc = fn(&str) -> Result<(String, String), Box<dyn Error>>;
const DAYS: &'static [DayFunc] = &[day1::solve, day2::solve];

fn main() {
    let inputs_cache_path =
        input::init_inputs_cache().expect("Failed to initialize inputs cache path!");
    let cookie_opt = input::load_cookie().ok();

    DAYS.iter()
        .enumerate()
        .map(|(i, day)| (i + 1, day))
        .map(|(day, day_func)| {
            let result = match input::retrieve_input(day, cookie_opt.as_deref(), &inputs_cache_path)
            {
                Ok(input) => day_func(&input),
                Err(err) => Err(err.into()),
            };

            (day, result)
        })
        .for_each(|(day, result)| {
            print!("Day {} :: ", day);

            match result {
                Ok((part1, part2)) => println!("Part 1 = {}, Part 2 = {}", part1, part2),
                Err(err) => println!("Error! {:#?}", err),
            }
        });
}

#[allow(dead_code)]
pub(crate) fn not_yet_implemented() -> String {
    "Not yet implemented!".to_string()
}
