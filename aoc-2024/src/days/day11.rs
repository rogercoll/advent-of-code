use std::collections::HashMap;

fn day11(blinks: usize) -> i64 {
    let mut stones: HashMap<i64, i64> = include_str!("../../input/day11.txt")
        .trim_end()
        .split(" ")
        .fold(HashMap::new(), |mut acc, value| {
            acc.entry(value.parse().unwrap())
                .and_modify(|e| *e += 1)
                .or_insert(1);
            acc
        });

    for _ in 0..blinks {
        let current_stones: HashMap<i64, i64> = stones
            .iter()
            .filter_map(|(key, val)| {
                if *val > 0 {
                    return Some((*key, *val));
                }
                None
            })
            .collect();

        for (cstone, ncstones) in current_stones {
            *stones.get_mut(&cstone).unwrap() -= ncstones;
            if cstone == 0 {
                stones
                    .entry(1)
                    .and_modify(|e| *e += ncstones)
                    .or_insert(ncstones);
                continue;
            }
            let sstone = cstone.to_string();
            if sstone.len() % 2 == 0 {
                stones
                    .entry(sstone[0..sstone.len() / 2].parse().unwrap())
                    .and_modify(|e| *e += ncstones)
                    .or_insert(ncstones);
                stones
                    .entry(sstone[sstone.len() / 2..].parse().unwrap())
                    .and_modify(|e| *e += ncstones)
                    .or_insert(ncstones);
            } else {
                stones
                    .entry(cstone * 2024)
                    .and_modify(|e| *e += ncstones)
                    .or_insert(ncstones);
            }
        }
    }

    stones.values().filter(|val| **val > 0).sum::<i64>()
}

pub(crate) fn part1() {
    println!("{}", day11(25));
}

pub(crate) fn part2() {
    println!("{}", day11(75));
}
