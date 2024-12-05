use regex::Regex;

pub(crate) fn part1() {
    let re = Regex::new(r"mul\(([0-9]+)\,([0-9]+)\)").unwrap();
    let mut movies = vec![];
    for (_, [num1, num2]) in re
        .captures_iter(include_str!("../../input/day3.txt"))
        .map(|c| c.extract())
    {
        movies.push(num1.parse::<i64>().unwrap() * num2.parse::<i64>().unwrap());
    }
    println!("{:?}", movies.iter().sum::<i64>());
}

pub(crate) fn part2() {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();
    let mut enabled = true;

    println!(
        "{}",
        re.captures_iter(include_str!("../../input/day3.txt"))
            .filter(|capture| {
                if capture.get(0).unwrap().as_str() == "do()" {
                    enabled = true;
                    return false;
                } else if capture.get(0).unwrap().as_str() == "don't()" {
                    enabled = false;
                    return false;
                }
                return enabled;
            })
            .map(|mul_caputre| {
                mul_caputre.get(1).unwrap().as_str().parse::<i64>().unwrap()
                    * mul_caputre.get(2).unwrap().as_str().parse::<i64>().unwrap()
            })
            .sum::<i64>()
    )
}
