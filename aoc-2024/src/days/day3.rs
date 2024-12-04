use regex::Regex;

pub(crate) fn part1() {
    let re = Regex::new(r"mul\(([0-9]+)\,([0-9]+)\)").unwrap();
    let mut movies = vec![];
    for (_, [num1, num2]) in re
        .captures_iter(include_str!("../../input/day3.txt"))
        .map(|c| c.extract())
    {
        println!(
            "{}, {}",
            num1.parse::<i64>().unwrap(),
            num2.parse::<i64>().unwrap()
        );
        movies.push(num1.parse::<i64>().unwrap() * num2.parse::<i64>().unwrap());
    }
    println!("{:?}", movies.iter().sum::<i64>());
}

pub(crate) fn part2() {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();

    println!(
        "{}",
        re.captures_iter(include_str!("../../input/day3.txt"))
            .fold((0, true), |(mut acc, mut enabled), capture| {
                if capture.get(0).unwrap().as_str() == "do()" {
                    enabled = true;
                } else if capture.get(0).unwrap().as_str() == "don't()" {
                    enabled = false;
                } else {
                    if enabled {
                        acc += capture.get(1).unwrap().as_str().parse::<i64>().unwrap()
                            * capture.get(2).unwrap().as_str().parse::<i64>().unwrap()
                    }
                }
                (acc, enabled)
            })
            .0
    )
}
