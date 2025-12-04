use std::collections::HashSet;

const MOVES: [(isize, isize); 8] = [
    (0, -1),
    (0, 1),
    (-1, 0),
    (1, 0),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];

fn part1() {
    let map = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let (height, width) = (map.len() as isize, map[0].len() as isize);
    println!(
        "Part 1: {}",
        (0..map[0].len() as isize)
            .flat_map(|x| (0..map.len() as isize).map(move |y| (x, y)))
            .filter(|(x, y)| {
                let get_val = |x: isize, y: isize| -> u8 {
                    map.get(y as usize)
                        .and_then(|row| row.get(x as usize).copied())
                        .unwrap_or_default()
                };

                if get_val(*x, *y) == b'@' {
                    MOVES
                        .iter()
                        .map(|(movx, movy)| (x + movx, y + movy))
                        .filter(|c| {
                            (c.0 >= 0 && c.0 < height && c.1 >= 0 && c.1 < width)
                                && get_val(c.0, c.1) == b'@'
                        })
                        .count()
                        < 4
                } else {
                    false
                }
            })
            .count()
    )
}

fn part2() {
    let map = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let (height, width) = (map.len() as isize, map[0].len() as isize);
    let scan = |removed: &mut HashSet<(isize, isize)>| -> Vec<(isize, isize)> {
        (0..map[0].len() as isize)
            .flat_map(|x| (0..map.len() as isize).map(move |y| (x, y)))
            .filter_map(|(x, y)| {
                let get_val = |x: isize, y: isize| -> u8 {
                    map.get(y as usize)
                        .and_then(|row| row.get(x as usize).copied())
                        .unwrap_or_default()
                };

                if get_val(x, y) == b'@' && removed.get(&(x, y)).is_none() {
                    if MOVES
                        .iter()
                        .map(|(movx, movy)| (x + movx, y + movy))
                        .filter(|c| {
                            (c.0 >= 0 && c.0 < height && c.1 >= 0 && c.1 < width)
                                && get_val(c.0, c.1) == b'@'
                                && removed.get(&(c.0, c.1)).is_none()
                        })
                        .count()
                        < 4
                    {
                        removed.insert((x, y));
                        return Some((x, y));
                    }
                }
                None
            })
            .collect()
    };
    let mut removed = HashSet::new();
    let (mut c_removed, mut total) = (scan(&mut removed), 0);
    while c_removed.len() > 0 {
        total += c_removed.len();
        c_removed = scan(&mut removed);
    }
    println!("Part 2: {}", total);
}

fn main() {
    part1();
    part2();
}
