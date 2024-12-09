use super::{DayResult, PartResult};
use itertools::Itertools;
use rustc_hash::FxHashSet;

pub(crate) fn solve(input: &str) -> DayResult {
    Ok((part1(input)?, part2(input)?))
}

fn part1(input: &str) -> PartResult {
    // 1. Parse input into mut Vec<Option<usize>>, where the usize represents the ID (incrementing).
    // 2. Determine the first free space.
    // 3. Iterate backwards, moving each block to the free space, and finding the next free space.
    // 4. Checksum is given by sum of enumerating each block * their ID.

    let mut fs: Vec<Option<usize>> = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|char| char.to_digit(10).unwrap())
        .enumerate()
        .flat_map(|(index, count)| {
            let block = match index % 2 {
                0 => Some(index / 2),
                _ => None,
            };

            vec![block; count as usize]
        })
        .collect();

    print_fs(&fs);

    while let Some(free_index) = fs.iter().position(|block| block.is_none()) {
        let Some(swap_block_index) = fs
            .iter()
            .enumerate()
            .rev()
            .find(|(_, block)| block.is_some())
            .map(|(index, _)| index)
        else {
            break;
        };

        if swap_block_index < free_index {
            break;
        }

        fs.swap(swap_block_index, free_index);
    }

    print_fs(&fs);

    Ok(fs
        .iter()
        .enumerate()
        .filter_map(|(index, block)| block.map(|id| index * id))
        .sum::<usize>()
        .to_string())
}

fn part2(input: &str) -> PartResult {
    let mut fs: Vec<Block> = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|char| char.to_digit(10).unwrap())
        .enumerate()
        .map(|(index, size)| match index % 2 {
            0 => Block::new_file(index / 2, size),
            _ => Block::new_free(size),
        })
        .collect();

    let mut seen_ids = FxHashSet::<usize>::default();
    let mut file_index = fs.len() - 1;

    while file_index > 0 {
        let file = fs[file_index];

        let Some(id) = file.id else {
            file_index -= 1;
            continue;
        };

        if seen_ids.contains(&id) {
            file_index -= 1;
            continue;
        }

        seen_ids.insert(id);

        let Some(free_index) = fs
            .iter()
            .position(|free| free.id.is_none() && free.size >= file.size)
        else {
            file_index -= 1;
            continue;
        };

        if free_index > file_index {
            file_index -= 1;
            continue;
        }

        let free = fs[free_index];
        let size_diff = free.size - file.size;

        if size_diff > 0 {
            let (remaining_free, displaced_free) = free.split(size_diff);
            let _ = std::mem::replace(&mut fs[free_index], file);
            let _ = std::mem::replace(&mut fs[file_index], displaced_free);
            fs.insert(free_index + 1, remaining_free);
        } else {
            fs.swap(file_index, free_index);
            file_index -= 1;
        }
    }

    // println!(
    //     "\n\nd2 ::  {}",
    //     fs.iter()
    //         .map(|block| match block.id {
    //             Some(id) => id.to_string(),
    //             None => String::from("."),
    //         }
    //         .repeat(block.size as usize))
    //         .join("")
    // );

    let mut sum = 0;
    let mut offset = 0;

    for block in fs {
        if let Some(id) = block.id {
            for index in 0..(block.size as usize) {
                sum += (offset + index) * id;
            }
        }

        offset += block.size as usize;
    }

    Ok(sum.to_string())
}

fn print_fs(fs: &[Option<usize>]) {
    // println!(
    //     "{}",
    //     fs.iter()
    //         .map(|block| match block {
    //             Some(index) => index.to_string(),
    //             None => String::from("."),
    //         })
    //         .join("")
    // );
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
struct Block {
    id: Option<usize>,
    size: u32,
}

impl Block {
    fn new_file(id: usize, size: u32) -> Self {
        Block { id: Some(id), size }
    }

    fn new_free(size: u32) -> Self {
        Block {
            size,
            ..Default::default()
        }
    }

    fn split(&self, pivot: u32) -> (Self, Self) {
        debug_assert!(pivot < self.size);

        let left_size = pivot;
        let right_size = self.size - pivot;

        (
            Block {
                id: self.id,
                size: left_size,
            },
            Block {
                id: self.id,
                size: right_size,
            },
        )
    }
}
