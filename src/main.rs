
mod input;
mod day1;

use core::str;
use std::{error::Error, fs::{create_dir, File}, io::Read, path::Path};

type DayFunc = fn(&str) -> Result<String, Box<dyn Error>>;
const DAYS: &'static [DayFunc] = &[day1::d1];

const INPUTS_DIR_PATH: &str = "inputs";
const AOC_COOKIE_PATH: &str = "cookie.txt";

fn main() {
	
	let inputs_cache_path = Path::new(INPUTS_DIR_PATH);
	if !inputs_cache_path.exists() {
		create_dir(inputs_cache_path).expect("Failed to create the input cache directory!");
	}
	
	let download_cookie = File::open(AOC_COOKIE_PATH).map(|mut file| {

		let mut string = String::new();

		file
			.read_to_string(&mut string)
			.map(|_| string)
			.unwrap_or_else(|_| panic!("Failed to read the contents of `{}`!", AOC_COOKIE_PATH))
		
	}).ok();

	DAYS
		.iter()
		.enumerate()
		.map(|(i, day)| (i + 1, day))
		.map(|(day, day_func)| {

			let result = match input::retrieve_input(day, download_cookie.as_deref(), inputs_cache_path) {
				Ok(input) => day_func(&input),
				Err(err) => Err(err.into()),
			};

			(day, result)

		}).for_each(|(day, result)| {

			print!("Day {} :: ", day);

			match result {
				Ok(output) => println!("Output = {}", output),
				Err(err) => println!("Error! {:#?}", err),
			}

		});

}
