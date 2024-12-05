use std::isize;

pub(crate) fn part1() {
    let map = include_bytes!("../../input/day4.txt")
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    println!(
        "{}",
        (0..map[0].len() as isize)
            .flat_map(|x| (0..map.len() as isize).map(move |y| (x, y)))
            .flat_map(|(x, y)| {
                [
                    [(x, y), (x + 1, y - 1), (x + 2, y - 2), (x + 3, y - 3)], // NE
                    [(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],             // E
                    [(x, y), (x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3)], // SE
                    [(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],             // S
                ]
            })
            .filter(|cords| {
                let word = cords
                    .iter()
                    .map(|(x, y)| {
                        map.get(*y as usize)
                            .and_then(|row| row.get(*x as usize).copied())
                            .unwrap_or_default()
                    })
                    .collect::<Vec<_>>();
                word == b"XMAS" || word == b"SAMX"
            })
            .count()
    );
}

pub(crate) fn part2() {
    let map = include_bytes!("../../input/day4.txt")
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    println!(
        "{}",
        (0..map[0].len() as isize)
            .flat_map(|x| (1..map.len() as isize - 1).map(move |y| (x, y)))
            .map(|(x, y)| {
                // diagonals
                (
                    [(x + 1, y - 1), (x, y), (x - 1, y + 1)],
                    [(x + 1, y + 1), (x, y), (x - 1, y - 1)],
                )
            })
            .map(|(d1, d2)| {
                let to_word = |diagonal: [(isize, isize); 3]| -> Vec<u8> {
                    diagonal
                        .iter()
                        .map(|(x, y)| {
                            map.get(*y as usize)
                                .and_then(|row| row.get(*x as usize).copied())
                                .unwrap_or_default()
                        })
                        .collect::<Vec<_>>()
                };
                (to_word(d1), to_word(d2))
            })
            .filter(|(d1, d2)| {
                let xmas = |word: &[u8]| -> bool { word == b"MAS" || word == b"SAM" };
                xmas(d1.as_slice()) && xmas(d2.as_slice())
            })
            .count()
    );
}
