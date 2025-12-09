use std::collections::HashSet;

fn part1() {
    let map = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let get_val = |x: isize, y: isize| -> u8 {
        map.get(y as usize)
            .and_then(|row| row.get(x as usize).copied())
            .unwrap_or_default()
    };

    let (height, width) = (map.len() as isize, map[0].len() as isize);
    let valid_pos =
        |x: isize, y: isize| -> bool { return x >= 0 && x < height && y >= 0 && y < width };

    let mut next_pos = Vec::from([(map[0].iter().position(|&b| b == b'S').unwrap() as isize, 0)]);

    let mut beams = HashSet::new();
    let mut counter = 0;
    while let Some((x, mut y)) = next_pos.pop() {
        match get_val(x, y) {
            b'.' | b'S' => {
                y += 1;
                if valid_pos(x, y) && !beams.contains(&(x, y)) {
                    next_pos.push((x, y));
                    beams.insert((x, y));
                }
            }
            b'^' => {
                for (x, y) in [(x - 1, y), (x + 1, y)] {
                    if valid_pos(x, y) && !beams.contains(&(x, y)) {
                        next_pos.push((x, y));
                        beams.insert((x, y));
                    }
                }
                counter += 1;
            }
            _ => unreachable!(),
        }
    }
    println!("Part 1: {}", counter);
}

fn part2() {
    let map = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    // fixed row length
    let mut paths = [0; 141];
    paths[map[0].iter().position(|&b| b == b'S').unwrap()] = 1;

    for row in map[1..].iter() {
        for (i, count) in paths.into_iter().enumerate() {
            if row[i] == b'^' {
                paths[i] = 0;
                paths[i - 1] += count;
                paths[i + 1] += count;
            }
        }
    }
    println!("Part 2: {}", paths.iter().sum::<usize>());
}

fn main() {
    part1();
    part2();
}
