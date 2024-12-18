
use super::{DayResult, PartResult};
use itertools::Itertools;
use rustc_hash::FxHashSet;

pub(crate) fn solve(input: &str) -> DayResult {
    let fs = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|char| char.to_digit(10).unwrap() as usize)
        .collect_vec();
    Ok((part1(&fs)?, part2(input)?))
}

fn part1(fs: &[usize]) -> PartResult {
    // 1. Iterate forward and backwards over the FS blocks.
    // 2. Pour the block's contents into the free block. If it runs out of space, move to the next
    //    free block.
    //
    //    For each iteration of this, we'll be adding to the checksum.

    let mut checksum: usize = 0;
    let mut free_index: usize = 1;
    let mut block_index: usize = fs.len() - 1;

    if block_index % 2 != 0 {
        block_index -= 1;
    }

    let mut free_size = fs[free_index];
    let mut block_size = fs[block_index];

    // Initial block offsets things.
    let mut free_offset = fs[0];

    'big_loop: loop {
        while free_size == 0 {
            // Move to the next free block.
            free_index += 2;

            if free_index >= fs.len() {
                break 'big_loop;
            }

            let skipped_block_index = free_index - 1;
            let skipped_block_id = skipped_block_index / 2;
            let mut skipped_block_size = fs[free_index - 1];

            if skipped_block_index == block_index {
                skipped_block_size = block_size;
            }

            for _ in 0..skipped_block_size {
                checksum += free_offset * skipped_block_id;
                free_offset += 1;
            }

            free_size = fs[free_index];
        }

        while block_size == 0 {
            // Move to the next block to be moved.
            block_index -= 2;

            if block_index == 0 {
                break 'big_loop;
            }

            block_size = fs[block_index];
        }

        if free_index >= block_index {
            break;
        }

        if block_index == 0 {
            break;
        }

        checksum += free_offset * (block_index / 2);
        block_size -= 1;
        free_size -= 1;
        free_offset += 1;
    }

    Ok(checksum.to_string())
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
