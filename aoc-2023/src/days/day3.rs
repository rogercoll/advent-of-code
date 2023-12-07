use std::collections::HashMap;

fn read_grid(input: String) -> Vec<Vec<u8>> {
    input
        .split('\n')
        .map(|line| line.as_bytes().iter().map(|c| *c).collect::<Vec<u8>>())
        .collect()
}

const MAX_DIGITS: usize = 4;

pub fn part1(input: String) -> String {
    let grid = read_grid(input);
    let width = grid[0].len() as isize;

    let grid: Vec<u8> = grid.into_iter().flatten().collect();

    (0..grid.len())
        .filter(|i| {
            // get index of first digit
            grid[*i].is_ascii_digit()
                && !grid
                    .get(i.wrapping_sub(1))
                    .map_or(false, u8::is_ascii_digit)
        })
        .map(|i| {
            // get number and digits
            let digits = (i + 1..i + MAX_DIGITS)
                .position(|x| !grid[x].is_ascii_digit())
                .unwrap_or_else(|| 2)
                + 1;
            (
                i,
                digits as isize,
                atoi::atoi::<usize>(&grid[i..i + digits]).unwrap(),
            )
        })
        .filter(|(i, digits, _number)| {
            (-width - 1..-width + *digits + 1)
                .chain([-1, *digits])
                .chain(width - 1..width + *digits + 1)
                .any(|j| {
                    grid.get((*i as isize + j) as usize)
                        .map_or(false, |b| b != &b'.' && b.is_ascii_punctuation())
                })
        })
        .map(|vals| vals.2)
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let grid = read_grid(input);
    let width = grid[0].len() as isize;

    let grid: Vec<u8> = grid.into_iter().flatten().collect();

    (0..grid.len())
        .filter(|i| {
            // get index of first digit
            grid[*i].is_ascii_digit()
                && !grid
                    .get(i.wrapping_sub(1))
                    .map_or(false, u8::is_ascii_digit)
        })
        .map(|i| {
            // get number and digits
            let digits = (i + 1..i + MAX_DIGITS)
                .position(|x| !grid[x].is_ascii_digit())
                .unwrap_or_else(|| 2)
                + 1;
            (
                i,
                digits as isize,
                atoi::atoi::<usize>(&grid[i..i + digits]).unwrap(),
            )
        })
        .map(|(i, digits, number)| {
            let star_position: Vec<isize> = (-width - 1..-width + digits + 1)
                .chain([-1, digits])
                .chain(width - 1..width + digits + 1)
                .map(|pos| i as isize + pos)
                .filter(|j| grid.get(*j as usize).map_or(false, |b| b == &b'*'))
                .collect();
            (number, star_position)
        })
        .fold(HashMap::new(), |mut acc, (number, star_position)| {
            star_position.into_iter().for_each(|star| {
                acc.entry(star)
                    .and_modify(|stars: &mut Vec<usize>| stars.push(number))
                    .or_insert(Vec::from([number]));
            });
            acc
        })
        .into_values()
        .filter(|gears| gears.len() == 2)
        .map(|ratios| ratios.iter().product::<usize>())
        .sum::<usize>()
        .to_string()
}
