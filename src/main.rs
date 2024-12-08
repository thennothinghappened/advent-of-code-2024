use std::{error::Error, time::Instant};

mod days;
mod input;
mod utils;

fn main() {
    let inputs_cache_path =
        input::init_inputs_cache().expect("Failed to initialize inputs cache path!");
    let cookie_opt = input::load_cookie().ok();

    days::DAYS
        .iter()
        .enumerate()
        .map(|(i, day)| (i + 1, day))
        .map::<Result<_, Box<dyn Error>>, _>(|(day, day_func)| {
            println!();
            println!("--- Day {} ---", day);

            let input = input::retrieve_input(day, cookie_opt.as_deref(), &inputs_cache_path)?;
            let now = Instant::now();
            let result = day_func(&input)?;

            Ok((result, now.elapsed()))
        })
        .for_each(|result| match result {
            Ok(((part1, part2), elapsed)) => {
                println!("Part 1 :: {}", part1);
                println!("Part 2 :: {}", part2);
                println!("Took {}µs", elapsed.as_micros());
            }
            Err(err) => println!("Error! {:#?}", err),
        });
}
