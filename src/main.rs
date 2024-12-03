use core::str;
use std::{fmt::Write, fs::{create_dir, File}, io::{Read, Write as OtherWrite}, path::Path};
use curl::easy::Easy;

type DayFunc = fn(&str);
const DAYS: &'static [DayFunc] = &[d1];

fn main() {

	let aoc_cookie = match File::open("cookie.txt") {
		Ok(mut file) => {
			
			let mut string = String::new();
			file.read_to_string(&mut string).expect("Failed to read cookie!");

			println!("cookie: {}", string);

			string

		},
		Err(_) => panic!("Please supply a cookie via `cookie.txt`, so we can download AOC day inputs."),
	};

	let cache_path = Path::new("./cache");

	if !cache_path.exists() {
		create_dir(cache_path).expect("Failed to create the cache directory!");
	}

	for (index, day) in DAYS.iter().enumerate() {
		
		let day_number = index + 1;
		let input_path = cache_path.join(format!("day_{}.txt", day_number));
		
		let input = match File::open(&input_path) {

			Ok(mut file) => {

				let mut string = String::new();

				file
					.read_to_string(&mut string)
					.expect("Failed to read input data from cached input file!");

				string

			},

			Err(_) => {

				println!("Downloading input for day {}...", day_number);
				
				let mut input = String::new();
				let mut request = Easy::new();

				request
					.url(&format!("https://adventofcode.com/2024/day/{}/input", day_number))
					.unwrap();

				request
					.cookie(&aoc_cookie)
					.unwrap();

				{

					let mut transfer = request.transfer();
					
					transfer.write_function(|data| {

						input.write_str(str::from_utf8(data).unwrap())
							.map(|_| Ok(input.len()))
							.expect("Very bad things have happened while writing input to a string!!!")

					}).unwrap();

					transfer
						.perform()
						.expect(&format!("Failed to get input for day {}!", day_number));

				}
				
				let mut file = File::create_new(&input_path)
					.expect("Failed to create file for day input!");

				file.write_all(input.as_bytes())
					.expect("Failed to write data to input cache file!");

				input
				
			}

		};

		day(&input);

	}

}

#[allow(unused)]
fn d1(input: &str) {

	// 1. Create two arrays.
	// 2. Iterate over each line of input, putting LHS of whitespace into arr1, RHS into arr2 (converted to numbers.)
	// 3. Get the smallest of both arrays, += the absolute difference to output.
	// 4. Print output.

	let mut arr1 = Vec::<i32>::new();
	let mut arr2 = Vec::<i32>::new();
	let mut sum = 0;
	
	for line in input.lines() {

		let mut split = line.split_whitespace();
		let lhs = split.next().unwrap().parse::<i32>().unwrap();
		let rhs = split.next().unwrap().parse::<i32>().unwrap();
		
		arr1.push(lhs);
		arr2.push(rhs);

	}

	arr1.sort();
	arr2.sort();

	for (index, lhs) in arr1.iter().enumerate() {
		
		let rhs = arr2[index];
		sum += lhs.abs_diff(rhs);

	}

	println!("Result: {}", sum);

}
