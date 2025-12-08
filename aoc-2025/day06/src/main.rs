use std::ops;
fn part1() {
    let input = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .map(|nums| {
            nums.split(|&b| b == b' ')
                .filter(|num| !num.is_empty())
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    println!(
        "Part 1: {}",
        (0..input[0].len())
            .map(|i| {
                let row_nums = input[..input.len() - 1]
                    .iter()
                    .map(|row| atoi::atoi::<usize>(row[i]).unwrap());
                if input.last().unwrap()[i][0] == b'+' {
                    row_nums.sum::<usize>()
                } else {
                    row_nums.product()
                }
            })
            .sum::<usize>()
    )
}

fn part2() {
    let mut numbers = include_str!("../input.txt").lines();
    let operations = numbers.next_back().unwrap();

    let numbers = numbers
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).map(u64::from))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut numbers = (0..numbers[0].len()).rev().map(|x| {
        (0..numbers.len())
            .map(|y| numbers[y][x])
            .reduce(|l, r| match (l, r) {
                (None, None) => None,
                (None, v) | (v, None) => v,
                (Some(l), Some(r)) => Some(l * 10 + r),
            })
            .flatten()
    });

    println!(
        "Part 2: {}",
        operations
            .split_ascii_whitespace()
            .rev()
            .map(|op| {
                numbers
                    .by_ref()
                    .take_while(|n| n.is_some())
                    .flatten()
                    .reduce({
                        if op == "+" {
                            ops::Add::add
                        } else {
                            ops::Mul::mul
                        }
                    })
                    .unwrap()
            })
            .sum::<u64>()
    )
}

fn main() {
    part1();
    part2();
}
