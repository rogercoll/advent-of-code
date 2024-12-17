use std::{
    collections::{HashSet, VecDeque},
    isize,
};

pub(crate) fn part1() {
    let map = include_bytes!("../../input/day10.txt")
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
                    .unwrap_or_default()
                    == b'0'
            })
            .map(|(x, y)| {
                let (height, width) = (map.len() as isize, map[0].len() as isize);
                let moves = [(0, -1), (0, 1), (-1, 0), (1, 0)];
                let mut pos_moves = VecDeque::from([((0, (x, y)))]);
                let mut total9 = HashSet::new();
                while let Some(next) = pos_moves.pop_front() {
                    if next.0 == 9 {
                        total9.insert(next.1);
                        continue;
                    }
                    pos_moves.extend(
                        moves
                            .iter()
                            .map(|(movx, movy)| (next.1 .0 + movx, next.1 .1 + movy))
                            .filter(|c| {
                                (c.0 >= 0 && c.0 < height && c.1 >= 0 && c.1 < width)
                                    && map
                                        .get(c.1 as usize)
                                        .and_then(|row| row.get(c.0 as usize).copied())
                                        .unwrap_or_default()
                                        - b'0'
                                        == next.0 + 1
                            })
                            .map(|c| {
                                (
                                    map.get(c.1 as usize)
                                        .and_then(|row| row.get(c.0 as usize).copied())
                                        .unwrap_or_default()
                                        - b'0',
                                    (c.0, c.1),
                                )
                            }),
                    )
                }
                total9.len()
            })
            .sum::<usize>()
    );
}

pub(crate) fn part2() {
    let map = include_bytes!("../../input/day10.txt")
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
                    .unwrap_or_default()
                    == b'0'
            })
            .map(|(x, y)| {
                let (height, width) = (map.len() as isize, map[0].len() as isize);
                let moves = [(0, -1), (0, 1), (-1, 0), (1, 0)];
                let mut pos_moves = VecDeque::from([((0, (x, y)))]);
                let mut total9 = 0;
                while let Some(next) = pos_moves.pop_front() {
                    if next.0 == 9 {
                        total9 += 1;
                        continue;
                    }
                    pos_moves.extend(
                        moves
                            .iter()
                            .map(|(movx, movy)| (next.1 .0 + movx, next.1 .1 + movy))
                            .filter(|c| {
                                (c.0 >= 0 && c.0 < height && c.1 >= 0 && c.1 < width)
                                    && map
                                        .get(c.1 as usize)
                                        .and_then(|row| row.get(c.0 as usize).copied())
                                        .unwrap_or_default()
                                        - b'0'
                                        == next.0 + 1
                            })
                            .map(|c| {
                                (
                                    map.get(c.1 as usize)
                                        .and_then(|row| row.get(c.0 as usize).copied())
                                        .unwrap_or_default()
                                        - b'0',
                                    (c.0, c.1),
                                )
                            }),
                    )
                }
                total9
            })
            .sum::<usize>()
    );
}
