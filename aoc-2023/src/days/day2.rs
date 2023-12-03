fn str_to_digit(input: &str) -> usize {
    input.parse::<usize>().unwrap()
}
pub fn part1(input: String) -> String {
    input
        .lines()
        .map(|line| {
            let (game, cubes) = line.split_once(':').unwrap();
            (
                (str_to_digit(game.split_once(' ').unwrap().1)),
                cubes.split(';').fold((0, 0, 0), |mut acc, x| {
                    x.split(',').for_each(|cube| {
                        match cube[1..].split_once(' ').unwrap() {
                            (x, "red") => acc.0 = acc.0.max(str_to_digit(x)),
                            (x, "green") => acc.1 = acc.1.max(str_to_digit(x)),
                            (x, "blue") => acc.2 = acc.2.max(str_to_digit(x)),
                            (_, _) => unreachable!("invalid cubes"),
                        };
                    });
                    acc
                }),
            )
        })
        .filter(|(_, cubes)| cubes.0 <= 12 && cubes.1 <= 13 && cubes.2 <= 14)
        .map(|(game, _)| game)
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    input
        .lines()
        .map(|line| {
            let (_, cubes) = line.split_once(':').unwrap();
            cubes.split(';').fold((0, 0, 0), |mut acc, x| {
                x.split(',').for_each(|cube| {
                    match cube[1..].split_once(' ').unwrap() {
                        (x, "red") => acc.0 = acc.0.max(str_to_digit(x)),
                        (x, "green") => acc.1 = acc.1.max(str_to_digit(x)),
                        (x, "blue") => acc.2 = acc.2.max(str_to_digit(x)),
                        (_, _) => unreachable!("invalid cubes"),
                    };
                });
                acc
            })
        })
        .map(|cubes| cubes.0 * cubes.1 * cubes.2)
        .sum::<usize>()
        .to_string()
}
