use std::ops::Add;

type Position = (isize, isize);
fn part1() {
    let positions = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let nums = line
                .split(|b| *b == b',')
                .map(|num| atoi::atoi::<isize>(num).unwrap())
                .collect::<Vec<isize>>();
            (nums[0], nums[1])
        })
        .collect::<Vec<Position>>();

    let mut max_area = 0;
    (0..positions.len()).for_each(|i| {
        (0..positions.len()).skip(i + 1).for_each(|j| {
            max_area = std::cmp::max(
                max_area,
                positions[i].0.abs_diff(positions[j].0).add(1)
                    * positions[i].1.abs_diff(positions[j].1).add(1),
            )
        });
    });
    println!("Part 1: {}", max_area);
}

fn main() {
    part1();
}
