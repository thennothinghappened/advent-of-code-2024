use crate::utils::not_yet_implemented;
use std::error::Error;

const XMAS: &[char] = &['X', 'M', 'A', 'S'];

#[rustfmt::skip]
const CHECK_OFFSETS: &[(i32, i32)] = &[
	(-1, -1),	(0, -1),	(1, -1),
	(-1,  0),				(1,  0),
	(-1,  1),	(0,  1),	(1,  1),
];

pub(crate) fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    Ok((part1(input)?, part2(input)?))
}

fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let mut matches: usize = 0;

    // 1. Iterate over line, and each character.
    // 2. If the character is not the start of our pattern, ignore it.
    // 3. Scan the 3x3 chars around it. For each that match the next char, continue until the whole
    //    match is found, or it isn't.
    // 4. If a match was found, increment matches.

    let char_matrix = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    for y in 0..char_matrix.len() {
        for x in 0..char_matrix[y].len() {
            'try_offsets: for (ox, oy) in CHECK_OFFSETS {
                for i in 0..XMAS.len() {
                    let Ok(check_x) = usize::try_from((x as i32) + (ox * (i as i32))) else {
                        continue 'try_offsets;
                    };

                    let Ok(check_y) = usize::try_from((y as i32) + (oy * (i as i32))) else {
                        continue 'try_offsets;
                    };

                    if check_y >= char_matrix.len() {
                        continue 'try_offsets;
                    }

                    if check_x >= char_matrix[check_y].len() {
                        continue 'try_offsets;
                    }

                    let c = char_matrix[check_y][check_x];

                    if c != XMAS[i] {
                        continue 'try_offsets;
                    }
                }

                matches += 1;
            }
        }
    }

    Ok(matches.to_string())
}

fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    not_yet_implemented()
}
