mod day1;
mod input;

use core::str;
use std::error::Error;

type DayFunc = fn(&str) -> Result<String, Box<dyn Error>>;
const DAYS: &'static [DayFunc] = &[day1::d1];

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
                Ok(output) => println!("Output = {}", output),
                Err(err) => println!("Error! {:#?}", err),
            }
        });
}
