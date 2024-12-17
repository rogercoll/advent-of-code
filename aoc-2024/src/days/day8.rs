use std::{
    collections::{HashMap, HashSet},
    isize,
    ops::{Add, Mul, Neg, Sub},
};

use itertools::Itertools;

pub(crate) fn part1() {
    let map = include_str!("../../input/day8.txt")
        .lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let antenas: HashMap<char, HashSet<(isize, isize)>> = map
        .iter()
        .enumerate()
        .flat_map(|(r, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| *ch != '.')
                .map(move |(c, ch)| (ch, (r as isize, c as isize)))
        })
        .fold(
            HashMap::new(),
            |mut antenas: HashMap<char, HashSet<(isize, isize)>>, (atype, (antenax, antenay))| {
                antenas
                    .entry(atype)
                    .and_modify(|val: &mut HashSet<(isize, isize)>| {
                        val.insert((antenax, antenay));
                    })
                    .or_insert(HashSet::from([(antenax, antenay)]));
                antenas
            },
        );

    let set = antenas
        .iter()
        .flat_map(|(ch, set)| {
            set.iter()
                .copied()
                .tuple_combinations()
                .flat_map(|(a, b)| {
                    // b -a
                    let diff = (b.0 - a.0, b.1 - a.1);
                    // (a -diff, b + diff)
                    [(a.0 - diff.0, a.1 - diff.1), (b.0 + diff.0, b.1 + diff.1)]
                })
                .map(move |c| (*ch, c))
        })
        .filter(|(_, c)| {
            // point is inside map
            c.0 >= 0 && c.0 < map.len() as isize && c.1 >= 0 && c.1 < map[0].len() as isize
        })
        .map(|(_, c)| c)
        .collect::<HashSet<_>>();

    println!("{}", set.len());
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord(pub isize, pub isize);

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul<isize> for Coord {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Neg for Coord {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1)
    }
}

pub(crate) fn part2() {
    let map = include_str!("../../input/day8.txt")
        .lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let antenas: HashMap<char, HashSet<Coord>> = map
        .iter()
        .enumerate()
        .flat_map(|(r, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| *ch != '.')
                .map(move |(c, ch)| (ch, (r as isize, c as isize)))
        })
        .fold(
            HashMap::new(),
            |mut antenas: HashMap<char, HashSet<Coord>>, (atype, (antenax, antenay))| {
                antenas
                    .entry(atype)
                    .and_modify(|val: &mut HashSet<Coord>| {
                        val.insert(Coord(antenax, antenay));
                    })
                    .or_insert(HashSet::from([Coord(antenax, antenay)]));
                antenas
            },
        );

    let (height, width) = (map.len() as isize, map[0].len() as isize);
    let set = antenas
        .iter()
        .flat_map(|(ch, set)| {
            set.iter()
                .copied()
                .tuple_combinations()
                .flat_map(|(a, b)| {
                    // b -a
                    let diff = b - a;
                    let make_iter = move |c: Coord, diff: Coord| {
                        (1..).map(move |i| c + diff * i).take_while(move |c| {
                            c.0 >= 0 && c.0 < height && c.1 >= 0 && c.1 < width
                        })
                    };

                    make_iter(a, diff).chain(make_iter(b, -diff))
                })
                .map(move |c| (*ch, c))
        })
        .filter(|(_, c)| {
            // point is inside map
            c.0 >= 0 && c.0 < height && c.1 >= 0 && c.1 < width
        })
        .map(|(_, c)| c)
        .collect::<HashSet<_>>();

    println!("{}", set.len());
}
