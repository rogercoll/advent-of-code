fn part1() {
    let (fresh_ids, ava_ids) = include_str!("../input.txt").split_once("\n\n").unwrap();

    let fresh_ids = fresh_ids
        .split("\n")
        .map(|range| {
            let (a, b) = range.split_once("-").unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect::<Vec<(u64, u64)>>();

    println!(
        "Part 1: {}",
        ava_ids
            .split("\n")
            .filter(|line| !line.is_empty())
            .filter(|ing| {
                let ing_id = ing.parse::<u64>().unwrap();
                fresh_ids
                    .iter()
                    .any(|range_ids| ing_id >= range_ids.0 && ing_id <= range_ids.1)
            })
            .count()
    )
}

fn part2() {
    let (fresh_ids, _) = include_str!("../input.txt").split_once("\n\n").unwrap();

    let mut fresh_ids = fresh_ids
        .split("\n")
        .map(|range| {
            let (a, b) = range.split_once("-").unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect::<Vec<(u64, u64)>>();
    fresh_ids.sort_by_key(|r| r.0);

    let mut new_ranges: Vec<(u64, u64)> = Vec::new();
    for range in &fresh_ids {
        if new_ranges.is_empty() {
            new_ranges.push((range.0, range.1));
            continue;
        }

        let last_index = new_ranges.len() - 1;
        let last_range = &mut new_ranges[last_index];

        if range.0 <= last_range.1 + 1 {
            last_range.1 = std::cmp::max(last_range.1, range.1);
        } else {
            new_ranges.push((range.0, range.1));
        }
    }

    println!(
        "Part 2: {}",
        new_ranges
            .iter()
            .map(|range| range.1 - range.0 + 1)
            .sum::<u64>()
    )
}

fn main() {
    part1();
    part2();
}
