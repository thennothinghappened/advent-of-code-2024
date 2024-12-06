use std::{
    collections::{HashMap, HashSet},
    iter,
};

use crate::utils::not_yet_implemented;

use super::{DayResult, PartResult};

pub(crate) fn solve(input: &str) -> DayResult {
    Ok((part1(input)?, part2(input)?))
}

fn part1(input: &str) -> PartResult {
    let mut lines = input.lines().into_iter();
    let mut dependencies = HashMap::<usize, HashSet<usize>>::new();

    lines
        .by_ref()
        .map_while(|line| {
            let mut split = line.split('|');

            let dependency = split.next()?.parse::<usize>().ok()?;
            let page = split.next()?.parse::<usize>().ok()?;

            Some((page, dependency))
        })
        .for_each(|(page, dependency)| {
            dependencies
                .entry(page)
                .or_insert_with(|| HashSet::new())
                .insert(dependency);
        });

    let correct = lines
        .map(|update| {
            update
                .split(',')
                .map(|page| page.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .filter_map(|update| {
            let mut seen_pages = HashSet::<&usize>::new();

            for page in update.iter() {
                seen_pages.insert(page);

                let Some(deps) = dependencies.get(page) else {
                    continue;
                };

                if deps
                    .iter()
                    .filter(|dep| update.contains(dep))
                    .any(|dep| !seen_pages.contains(dep))
                {
                    return None;
                }
            }

            Some(update)
        });

    let sum_middle_pages: usize = correct.map(|update| update[update.len() / 2]).sum();
    Ok(sum_middle_pages.to_string())
}

fn part2(input: &str) -> PartResult {
    not_yet_implemented()
}
