use std::error::Error;


pub(crate) fn d1(input: &str) -> Result<String, Box<dyn Error>> {

	// 1. Create two arrays.
	// 2. Iterate over each line of input, putting LHS of whitespace into arr1, RHS into arr2 (converted to numbers.)
	// 3. Get the smallest of both arrays, += the absolute difference to output.
	// 4. Print output.

	let mut arr1 = Vec::<i32>::new();
	let mut arr2 = Vec::<i32>::new();
	let mut sum = 0;
	
	for line in input.lines() {

		let mut split = line.split_whitespace();
		let lhs = split.next().unwrap().parse::<i32>()?;
		let rhs = split.next().unwrap().parse::<i32>()?;
		
		arr1.push(lhs);
		arr2.push(rhs);

	}

	arr1.sort();
	arr2.sort();

	for (index, lhs) in arr1.iter().enumerate() {
		
		let rhs = arr2[index];
		sum += lhs.abs_diff(rhs);

	}

	Ok(format!("{}", sum))

}
