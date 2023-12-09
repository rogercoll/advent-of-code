pub fn part1(input: String) -> String {
    let (times, distances) = input.split_once('\n').unwrap();

    let times = times[5..]
        .split_whitespace()
        .filter_map(|num| num.parse::<usize>().ok())
        .collect::<Vec<usize>>();

    let distances = distances[9..]
        .split_whitespace()
        .filter_map(|num| num.parse::<usize>().ok())
        .collect::<Vec<usize>>();

    times
        .iter()
        .zip(distances)
        .map(|(time, distance)| {
            (1..time - 1)
                .map(|hold| (*time - hold) * hold)
                .filter(|cdistance| cdistance > &distance)
                .count()
        })
        .product::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let (times, distances) = input.split_once('\n').unwrap();

    let time = times[5..]
        .split_whitespace()
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let distance = distances[9..]
        .split_whitespace()
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    (1..time - 1)
        .map(|hold| (time - hold) * hold)
        .filter(|cdistance| cdistance > &distance)
        .count()
        .to_string()
}
