use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    iter,
};

use super::{DayResult, PartResult};

pub(crate) fn solve(input: &str) -> DayResult {
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

fn part1(dependencies: &HashMap<usize, HashSet<usize>>, updates: &Vec<Vec<usize>>) -> PartResult {
    let correct = updates
        .iter()
        .filter(|update| is_valid(update, dependencies));
    let sum_middle_pages: usize = correct.map(|update| update[update.len() / 2]).sum();

    Ok(sum_middle_pages.to_string())
}

fn part2(dependencies: &HashMap<usize, HashSet<usize>>, updates: &Vec<Vec<usize>>) -> PartResult {
    // We're working under the assumption that there IS always a valid order, and work to achieve
    // that.

    let sum_middle_pages = updates
        .iter()
        .filter(|update| !is_valid(update, dependencies))
        .map(|update| update.clone())
        .map(|mut update| {
            update.sort_unstable_by(|a, b| {
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
            });

            update
        })
        .map(|update| update[update.len() / 2])
        .sum::<usize>();

    Ok(sum_middle_pages.to_string())
}

/// Determine whether an update list satisfies the dependencies.
fn is_valid(update: &Vec<usize>, dependencies: &HashMap<usize, HashSet<usize>>) -> bool {
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
            return false;
        }
    }

    true
}
