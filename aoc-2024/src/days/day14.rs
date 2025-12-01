use std::isize;

use regex::Regex;

pub(crate) fn part1() {
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut robots = include_str!("../../input/day14.txt")
        .trim_end()
        .split("\n")
        .map(|robot| {
            let captures = re.captures(robot).unwrap();
            (
                (
                    captures[1].parse::<isize>().unwrap(),
                    captures[2].parse::<isize>().unwrap(),
                ),
                (
                    captures[3].parse::<isize>().unwrap(),
                    captures[4].parse::<isize>().unwrap(),
                ),
            )
        })
        // .inspect(|s| println!("Robots: {s:?}"))
        .collect::<Vec<((isize, isize), (isize, isize))>>();

    for _ in 0..100 {
        for robot in robots.iter_mut() {
            let mut next_pos = (robot.0 .0 + robot.1 .0, robot.0 .1 + robot.1 .1);
            if next_pos.0 >= 101 {
                next_pos.0 = next_pos.0 - 101;
            } else if next_pos.0 < 0 {
                next_pos.0 = 101 + next_pos.0;
            }
            if next_pos.1 >= 103 {
                next_pos.1 = next_pos.1 - 103;
            } else if next_pos.1 < 0 {
                next_pos.1 = 103 + next_pos.1;
            }

            robot.0 = next_pos
        }
    }

    println!(
        "{}",
        robots
            .iter()
            .fold([0; 4], |mut acc, robot| {
                if robot.0 .0 < 50 && robot.0 .1 < 51 {
                    acc[0] += 1;
                } else if robot.0 .0 < 50 && robot.0 .1 > 51 {
                    acc[1] += 1;
                } else if robot.0 .0 > 50 && robot.0 .1 < 51 {
                    acc[2] += 1;
                } else if robot.0 .0 > 50 && robot.0 .1 > 51 {
                    acc[3] += 1;
                }
                acc
            })
            .iter()
            .product::<isize>()
    );
}

pub(crate) fn part2() {
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut robots = include_str!("../../input/day14.txt")
        .trim_end()
        .split("\n")
        .map(|robot| {
            let captures = re.captures(robot).unwrap();
            (
                (
                    captures[1].parse::<isize>().unwrap(),
                    captures[2].parse::<isize>().unwrap(),
                ),
                (
                    captures[3].parse::<isize>().unwrap(),
                    captures[4].parse::<isize>().unwrap(),
                ),
            )
        })
        // .inspect(|s| println!("Robots: {s:?}"))
        .collect::<Vec<((isize, isize), (isize, isize))>>();

    let mut found = false;
    let mut second = 0;
    while !found {
        let mut h_lines = [0; 103];
        for robot in robots.iter_mut() {
            let mut next_pos = (robot.0 .0 + robot.1 .0, robot.0 .1 + robot.1 .1);
            if next_pos.0 >= 101 {
                next_pos.0 = next_pos.0 - 101;
            } else if next_pos.0 < 0 {
                next_pos.0 = 101 + next_pos.0;
            }
            if next_pos.1 >= 103 {
                next_pos.1 = next_pos.1 - 103;
            } else if next_pos.1 < 0 {
                next_pos.1 = 103 + next_pos.1;
            }
            h_lines[next_pos.1 as usize] += 1;

            robot.0 = next_pos
        }
        if h_lines.iter().any(|h_line| *h_line > 20) {
            found = true;
        }
        second += 1;
    }

    println!("{}", second);
}
