use std::{
    collections::{HashMap, HashSet},
    iter,
};

use crate::utils::not_yet_implemented;

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
    // TODO: rewrite this godawful hell. Sure, this is meant for learning Rust so I can excuse some
    // bad quality code for the sake of learning, but this is just ridiculous.

    let incorrect = updates
        .iter()
        .filter(|update| !is_valid(update, dependencies));

    let mut sum_middle_pages: usize = 0;

    // We're working under the assumption that there IS always a valid order, and work to achieve
    // that.

    for update in incorrect {
        // 1. Iterate over each entry.
        // 2. Check if its dependencies are satisfied.
        // 3. If not, move the unsatisfied dependency before it.
        // 4. If step 3 occurred, restart at 1.
        // 5. Consider it sorted.

        let mut shuffleable_update = update.clone();

        'dep_loop: loop {
            let mut seen_pages = HashSet::<&usize>::new();

            for (i, page) in shuffleable_update.clone().iter().enumerate() {
                seen_pages.insert(page);

                let Some(deps) = dependencies.get(page) else {
                    continue;
                };

                for unmet in deps
                    .iter()
                    .filter(|dep| !seen_pages.contains(dep))
                    .filter_map(|dep| shuffleable_update.iter().position(|page| *page == *dep))
                {
                    let dep = shuffleable_update.remove(unmet);
                    shuffleable_update.insert(i, dep);

                    continue 'dep_loop;
                }
            }

            break 'dep_loop;
        }

        debug_assert!(is_valid(&shuffleable_update, dependencies));
        sum_middle_pages += shuffleable_update[update.len() / 2];
    }

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
