use std::collections::HashSet;

struct Direction(isize, isize, u8);

pub(crate) fn part1() {
    let map = include_bytes!("../../input/day6.txt")
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    println!(
        "{}",
        (0..map[0].len() as isize)
            .flat_map(|x| (0..map.len() as isize).map(move |y| (x, y)))
            .filter(|(x, y)| {
                map.get(*y as usize)
                    .and_then(|row| row.get(*x as usize).copied())
                    .unwrap()
                    == b'^'
            })
            .map(|(mut x, mut y)| {
                let mut direction = Direction(0, -1, b'^');
                let mut visited = HashSet::from([(x, y)]);
                while let Some(position) = map
                    .get((y + direction.1) as usize)
                    .and_then(|row| row.get((x + direction.0) as usize))
                {
                    match position {
                        b'#' => {
                            direction = match direction.2 {
                                b'^' => Direction(1, 0, b'>'),
                                b'v' => Direction(-1, 0, b'<'),
                                b'>' => Direction(0, 1, b'v'),
                                b'<' => Direction(0, -1, b'^'),
                                _ => unreachable!(),
                            };
                        }
                        _ => {
                            x += direction.0;
                            y += direction.1;
                            visited.insert((x, y));
                        }
                    };
                    // Additional logic for when the condition is true
                }
                visited.len()
            })
            .sum::<usize>()
    );
}

pub(crate) fn part2() {
    let map = include_bytes!("../../input/day6.txt")
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    println!(
        "{}",
        (0..map[0].len() as isize)
            .flat_map(|x| (0..map.len() as isize).map(move |y| (x, y)))
            .filter(|(x, y)| {
                map.get(*y as usize)
                    .and_then(|row| row.get(*x as usize).copied())
                    .unwrap()
                    == b'^'
            })
            .map(|(mut x, mut y)| {
                let (original_x, original_y) = (x, y);
                let mut direction = Direction(0, -1, b'^');
                let mut visited = HashSet::new();
                while let Some(position) = map
                    .get((y + direction.1) as usize)
                    .and_then(|row| row.get((x + direction.0) as usize))
                {
                    match position {
                        b'#' => {
                            direction = match direction.2 {
                                b'^' => Direction(1, 0, b'>'),
                                b'v' => Direction(-1, 0, b'<'),
                                b'>' => Direction(0, 1, b'v'),
                                b'<' => Direction(0, -1, b'^'),
                                _ => unreachable!(),
                            };
                        }
                        _ => {
                            x += direction.0;
                            y += direction.1;
                            visited.insert((x, y));
                        }
                    };
                }

                visited
                    .iter()
                    .filter(|obstacle| {
                        let (mut x, mut y) = (original_x, original_y);
                        let mut cycle = HashSet::new();
                        let mut direction = Direction(0, -1, b'^');
                        while let Some(position) = map
                            .get((y + direction.1) as usize)
                            .and_then(|row| row.get((x + direction.0) as usize))
                        {
                            if ((y + direction.1 == obstacle.1) && (x + direction.0 == obstacle.0))
                                || *position == b'#'
                            {
                                direction = match direction.2 {
                                    b'^' => Direction(1, 0, b'>'),
                                    b'v' => Direction(-1, 0, b'<'),
                                    b'>' => Direction(0, 1, b'v'),
                                    b'<' => Direction(0, -1, b'^'),
                                    _ => unreachable!(),
                                };
                            } else {
                                x += direction.0;
                                y += direction.1;
                                if cycle.get(&(x, y, direction.2)).is_some() {
                                    return true;
                                }
                                cycle.insert((x, y, direction.2));
                            }
                        }
                        false
                    })
                    .count()
            })
            .sum::<usize>()
    );
}
