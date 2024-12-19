use std::{error::Error, time::Instant};

use clap::Parser;

mod days;
mod input;
mod utils;

#[derive(clap::Parser)]
#[command(about)]
struct Cli {
    #[arg(short, long, value_name = "DAYS")]
    run: Option<Vec<usize>>,
}

fn main() {
    let cli = Cli::parse();

    let inputs_cache_path =
        input::init_inputs_cache().expect("Failed to initialize inputs cache path!");
    let cookie_opt = input::load_cookie().ok();

    days::DAYS
        .iter()
        .enumerate()
        .map(|(i, day_func)| (i + 1, day_func))
        .filter(|(day, _)| match &cli.run {
            Some(days) => days.contains(day),
            None => true,
        })
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
