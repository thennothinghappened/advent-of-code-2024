use std::{cmp::Ordering, collections::HashMap};

use super::{DayResult, PartResult};

pub(crate) fn solve(input: &str) -> DayResult {
    let mut lines = input.lines().into_iter();
    let mut dependencies = HashMap::<usize, Vec<usize>>::new();

    lines
        .by_ref()
        .map_while(|line| {
            let (lhs, rhs) = line.split_once('|')?;

            let dependency = lhs.parse::<usize>().ok()?;
            let page = rhs.parse::<usize>().ok()?;

            Some((page, dependency))
        })
        .for_each(|(page, dependency)| {
            dependencies
                .entry(page)
                .or_insert_with(|| Vec::new())
                .push(dependency);
        });

    let updates = lines
        .map(|update| {
            update
                .split(',')
                .map(|page| page.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    Ok((
        part1(&dependencies, &updates)?,
        part2(&dependencies, &updates)?,
    ))
}

fn part1(dependencies: &HashMap<usize, Vec<usize>>, updates: &[Vec<usize>]) -> PartResult {
    let correct = updates
        .into_iter()
        .filter(|update| is_sorted(update, dependencies));
    let sum_middle_pages: usize = correct.map(|update| update[update.len() / 2]).sum();

    Ok(sum_middle_pages.to_string())
}

fn part2(dependencies: &HashMap<usize, Vec<usize>>, updates: &[Vec<usize>]) -> PartResult {
    // We're working under the assumption that there IS always a valid order, and work to achieve
    // that.

    let sum_middle_pages = updates
        .into_iter()
        .filter(|update| !is_sorted(update, dependencies))
        .map(|update| update.clone())
        .map(|mut update| {
            update.sort_unstable_by(|a, b| sort_pages(dependencies, a, b));
            update
        })
        .map(|update| update[update.len() / 2])
        .sum::<usize>();

    Ok(sum_middle_pages.to_string())
}

fn sort_pages(dependencies: &HashMap<usize, Vec<usize>>, a: &usize, b: &usize) -> Ordering {
    if let Some(deps_for_a) = dependencies.get(a) {
        if deps_for_a.contains(b) {
            return Ordering::Greater;
        }
    }

    if let Some(deps_for_b) = dependencies.get(b) {
        if deps_for_b.contains(a) {
            return Ordering::Less;
        }
    }

    Ordering::Equal
}

/// Determine whether an update list satisfies the dependencies.
fn is_sorted(update: &[usize], dependencies: &HashMap<usize, Vec<usize>>) -> bool {
    update.is_sorted_by(|a, b| sort_pages(dependencies, a, b).is_lt())
}
