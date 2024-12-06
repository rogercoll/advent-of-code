use std::{cmp::Ordering, collections::HashMap};

pub(crate) fn part1() {
    let (rules, updates) = include_str!("../../input/day5.txt")
        .split_once("\n\n")
        .unwrap();

    let mut rules = rules
        .split("\n")
        .map(|rule| rule.split_once("|").unwrap())
        .fold(
            HashMap::new(),
            |mut orders: HashMap<usize, Vec<usize>>, (key, value)| {
                orders
                    .entry(key.parse::<usize>().unwrap())
                    .or_default()
                    .push(value.parse::<usize>().unwrap());
                orders
            },
        );

    rules.values_mut().for_each(|pages| pages.sort_unstable());

    println!(
        "{}",
        updates
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|update| update
                .split(",")
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>())
            .filter(|update| {
                update.iter().enumerate().all(|(i, val)| {
                    if let Some(rule) = rules.get(val) {
                        return update[0..i].iter().all(|j| rule.binary_search(j).is_err());
                    }
                    return true;
                })
            })
            .map(|update| *update.get(update.len() / 2).unwrap())
            .sum::<usize>()
    );
}

pub(crate) fn part2() {
    let (rules, updates) = include_str!("../../input/day5.txt")
        .split_once("\n\n")
        .unwrap();

    let mut rules = rules
        .split("\n")
        .map(|rule| rule.split_once("|").unwrap())
        .fold(
            HashMap::new(),
            |mut orders: HashMap<usize, Vec<usize>>, (key, value)| {
                orders
                    .entry(key.parse::<usize>().unwrap())
                    .or_default()
                    .push(value.parse::<usize>().unwrap());
                orders
            },
        );

    rules.values_mut().for_each(|pages| pages.sort_unstable());

    println!(
        "{}",
        updates
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|update| update
                .split(",")
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>())
            .filter(|update| {
                update.iter().enumerate().any(|(i, val)| {
                    if let Some(rule) = rules.get(val) {
                        return update[0..i].iter().any(|j| rule.binary_search(j).is_ok());
                    }
                    return false;
                })
            })
            .map(|mut update| {
                update.sort_unstable_by(|a, b| {
                    if rules
                        .get(a)
                        .is_some_and(|rule| rule.binary_search(b).is_ok())
                    {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                });
                update
            })
            .map(|update| *update.get(update.len() / 2).unwrap())
            .sum::<usize>()
    );
}
