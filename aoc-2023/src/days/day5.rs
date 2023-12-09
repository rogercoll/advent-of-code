use std::{ops::Range, usize::MAX};

pub fn part1(input: String) -> String {
    let (seeds, maps) = input.split_once("\n\n").unwrap();

    let seeds = seeds[6..]
        .split_whitespace()
        .filter_map(|num| num.parse::<usize>().ok())
        .collect::<Vec<usize>>();

    let maps: Vec<Vec<(Range<usize>, Range<usize>)>> = maps
        .split("\n\n")
        .into_iter()
        .map(|map| {
            map.split("\n")
                .skip(1)
                .take_while(|line| !line.is_empty())
                .map(|line| {
                    let entry: Vec<usize> = line
                        .split_whitespace()
                        .filter_map(|n| n.parse::<usize>().ok())
                        .collect();
                    (entry[0]..entry[0] + entry[2], entry[1]..entry[1] + entry[2])
                })
                .collect()
        })
        .collect();

    seeds
        .into_iter()
        .map(|seed| {
            maps.iter().fold(seed, |seed, map| {
                map.iter()
                    .find(|(_destination, source)| source.contains(&seed))
                    .map(|(destination, source)| destination.start + (seed - source.start))
                    .unwrap_or(seed)
            })
        })
        .min()
        .unwrap()
        .to_string()
}

pub fn part2(input: String) -> String {
    "todo".to_string()
    // let (seeds, maps) = input.split_once("\n\n").unwrap();

    // let seeds = seeds[6..]
    //     .split_whitespace()
    //     .filter_map(|num| num.parse::<usize>().ok())
    //     .collect::<Vec<usize>>();

    // let maps: Vec<Vec<(Range<usize>, Range<usize>)>> = maps
    //     .split("\n\n")
    //     .into_iter()
    //     .map(|map| {
    //         map.split("\n")
    //             .skip(1)
    //             .take_while(|line| !line.is_empty())
    //             .map(|line| {
    //                 let entry: Vec<usize> = line
    //                     .split_whitespace()
    //                     .filter_map(|n| n.parse::<usize>().ok())
    //                     .collect();
    //                 (entry[0]..entry[0] + entry[2], entry[1]..entry[1] + entry[2])
    //             })
    //             .collect()
    //     })
    //     .collect();

    // seeds
    //     .chunks(2)
    //     .into_iter()
    //     .fold(MAX, |mut min_loc, range| {
    //         (range[0]..range[0] + range[1])
    //             .into_iter()
    //             .for_each(|seed| {
    //                 let location = maps.iter().fold(seed, |seed, map| {
    //                     map.iter()
    //                         .find(|(_destination, source)| source.contains(&seed))
    //                         .map(|(destination, source)| destination.start + (seed - source.start))
    //                         .unwrap_or(seed)
    //                 });
    //                 min_loc = min_loc.min(location);
    //             });
    //         min_loc
    //     })
    //     .to_string()
}
