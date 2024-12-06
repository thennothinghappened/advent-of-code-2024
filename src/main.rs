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
        .map(|(day, day_func)| {
            println!("--- Day {} ---", day);

            match input::retrieve_input(day, cookie_opt.as_deref(), &inputs_cache_path) {
                Ok(input) => day_func(&input),
                Err(err) => Err(err.into()),
            }
        })
        .for_each(|result| {
            match result {
                Ok((part1, part2)) => {
                    println!("Part 1 :: {}", part1);
                    println!("Part 2 :: {}", part2);
                }
                Err(err) => println!("Error! {:#?}", err),
            }
            println!();
        });
}
