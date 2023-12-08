pub fn part1(input: String) -> String {
    input
        .lines()
        .map(|line| {
            let (_card, numbers) = line.split_once(':').unwrap();
            let (winning, all) = numbers.split_once('|').unwrap();
            (
                (_card),
                (
                    winning
                        .split_whitespace()
                        .filter_map(|num| num.parse::<usize>().ok())
                        .collect::<Vec<usize>>(),
                    all.split_whitespace()
                        .filter_map(|num| num.parse::<usize>().ok())
                        .collect::<Vec<usize>>(),
                ),
            )
        })
        .map(|(_card, (winning, all))| {
            let mut acc = Vec::new();
            all.into_iter().for_each(|val| {
                if winning.contains(&val) {
                    acc.push(val)
                }
            });
            acc
        })
        .map(|vals| 2usize.pow(vals.len() as u32) >> 1)
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let mut copies = [1usize; 256];

    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (_card, numbers) = line.split_once(':').unwrap();
            let (winning, all) = numbers.split_once('|').unwrap();
            (
                i,
                (
                    winning
                        .split_whitespace()
                        .filter_map(|num| num.parse::<usize>().ok())
                        .collect::<Vec<usize>>(),
                    all.split_whitespace()
                        .filter_map(|num| num.parse::<usize>().ok())
                        .collect::<Vec<usize>>(),
                ),
            )
        })
        .map(|(_i, (winning, all))| {
            let mut acc = Vec::new();
            all.into_iter().for_each(|val| {
                if winning.contains(&val) {
                    acc.push(val)
                }
            });
            (_i, acc)
        })
        .map(|(i, vals)| {
            let copy_val = copies[i];
            (i..i + vals.len()).for_each(|j| copies[j + 1] += copy_val);
            copy_val * vals.len() + 1
        })
        .sum::<usize>()
        .to_string()
}
