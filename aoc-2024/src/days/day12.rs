use std::{
    collections::{HashSet, VecDeque},
    isize,
};

use ordered_float::OrderedFloat;

pub(crate) fn part1() {
    let map = include_bytes!("../../input/day12.txt")
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut regions: Vec<HashSet<(isize, isize)>> = Vec::new();
    let (height, width) = (map.len() as isize, map[0].len() as isize);

    (0..map[0].len() as isize)
        .flat_map(|x| (0..map.len() as isize).map(move |y| (x, y)))
        .for_each(|(x, y)| {
            if !visited.contains(&(x, y)) {
                visited.insert((x, y));
                let mut pos_moves = VecDeque::from([((x, y))]);
                let region_key = map
                    .get(y as usize)
                    .and_then(|row| row.get(x as usize).copied())
                    .unwrap_or_default();
                let mut region: HashSet<(isize, isize)> = HashSet::from([(x, y)]);

                while let Some(next) = pos_moves.pop_front() {
                    pos_moves.extend(
                        [
                            (next.0, next.1 - 1),
                            (next.0, next.1 + 1),
                            (next.0 - 1, next.1),
                            (next.0 + 1, next.1),
                        ]
                        .iter()
                        .filter(|c| {
                            if (c.0 >= 0 && c.0 < height && c.1 >= 0 && c.1 < width)
                                && map
                                    .get(c.1 as usize)
                                    .and_then(|row| row.get(c.0 as usize).copied())
                                    .unwrap_or_default()
                                    == region_key
                                && !region.contains(c)
                            {
                                region.insert(**c);
                                visited.insert(**c);
                                return true;
                            }
                            return false;
                        })
                        .map(|c| (c.0, c.1)),
                    )
                }
                regions.push(region);
            }
        });

    let mut total_price = 0;
    for region in regions {
        total_price += region
            .iter()
            .map(|val| {
                let mut fences = 4;
                [(0, -1), (0, 1), (-1, 0), (1, 0)].iter().for_each(|mov| {
                    if region.contains(&(mov.0 + val.0, mov.1 + val.1)) {
                        fences -= 1;
                    }
                });
                fences
            })
            .sum::<usize>()
            * region.len() as usize
    }
    println!("{:?}", total_price);
}

pub(crate) fn part2() {
    let map = include_bytes!("../../input/day12.txt")
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut regions: Vec<HashSet<(isize, isize)>> = Vec::new();
    let (height, width) = (map.len() as isize, map[0].len() as isize);

    (0..map[0].len() as isize)
        .flat_map(|x| (0..map.len() as isize).map(move |y| (x, y)))
        .for_each(|(x, y)| {
            if !visited.contains(&(x, y)) {
                visited.insert((x, y));
                let mut pos_moves = VecDeque::from([((x, y))]);
                let region_key = map
                    .get(y as usize)
                    .and_then(|row| row.get(x as usize).copied())
                    .unwrap_or_default();
                let mut region: HashSet<(isize, isize)> = HashSet::from([(x, y)]);

                while let Some(next) = pos_moves.pop_front() {
                    pos_moves.extend(
                        [
                            (next.0, next.1 - 1),
                            (next.0, next.1 + 1),
                            (next.0 - 1, next.1),
                            (next.0 + 1, next.1),
                        ]
                        .iter()
                        .filter(|c| {
                            if (c.0 >= 0 && c.0 < height && c.1 >= 0 && c.1 < width)
                                && map
                                    .get(c.1 as usize)
                                    .and_then(|row| row.get(c.0 as usize).copied())
                                    .unwrap_or_default()
                                    == region_key
                                && !region.contains(c)
                            {
                                region.insert(**c);
                                visited.insert(**c);
                                return true;
                            }
                            return false;
                        })
                        .map(|c| (c.0, c.1)),
                    )
                }
                regions.push(region);
            }
        });

    let sides = |region: HashSet<(isize, isize)>| -> usize {
        let region: HashSet<(OrderedFloat<f32>, OrderedFloat<f32>)> = region
            .iter()
            .map(|(x, y)| (OrderedFloat(*x as f32), OrderedFloat(*y as f32)))
            .collect();

        let mut corner_candidates = HashSet::new();
        for (r, c) in &region {
            for (cr, cc) in [
                (r - 0.5, c - 0.5),
                (r + 0.5, c - 0.5),
                (r + 0.5, c + 0.5),
                (r - 0.5, c + 0.5),
            ] {
                corner_candidates.insert((OrderedFloat(cr), OrderedFloat(cc)));
            }
        }
        let mut corners = 0;
        for (cr, cc) in corner_candidates {
            let config = [
                region.contains(&(*cr - 0.5, *cc - 0.5)),
                region.contains(&(*cr + 0.5, *cc - 0.5)),
                region.contains(&(*cr + 0.5, *cc + 0.5)),
                region.contains(&(*cr - 0.5, *cc + 0.5)),
            ];
            let number = config.iter().filter(|val| **val).count();
            if number == 1 || number == 3 {
                corners += 1;
            } else if number == 2 {
                if config == [true, false, true, false] || config == [false, true, false, true] {
                    corners += 2;
                }
            }
        }

        corners
    };

    let mut total_price = 0;
    for region in regions {
        total_price += sides(region.clone()) * region.len() as usize
    }
    println!("{:?}", total_price);
}
