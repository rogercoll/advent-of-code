pub fn part1(input: String) -> String {
    let digit_lookup = |line: &[u8], i: usize| -> Option<usize> {
        line[i]
            .is_ascii_digit()
            .then_some((line[i] - b'0') as usize)
    };
    input
        .lines()
        .map(|line| {
            let line: &[u8] = line.as_bytes();
            (0..line.len()).find_map(|i| digit_lookup(line, i)).unwrap() * 10
                + (0..line.len())
                    .rev()
                    .find_map(|i| digit_lookup(line, i))
                    .unwrap()
        })
        .sum::<usize>()
        .to_string()
}

const NUMS: &[&[u8]] = &[
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

pub fn part2(input: String) -> String {
    let digit_lookup = |line: &[u8], i: usize| -> Option<usize> {
        line[i]
            .is_ascii_digit()
            .then_some((line[i] - b'0') as usize)
            .or(NUMS
                .iter()
                .enumerate()
                .find(|(_, name)| line[i..].starts_with(name))
                .map(|(num, _)| num + 1))
    };
    input
        .lines()
        .map(|line| {
            let line: &[u8] = line.as_bytes();
            (0..line.len()).find_map(|i| digit_lookup(line, i)).unwrap() * 10
                + (0..line.len())
                    .rev()
                    .find_map(|i| digit_lookup(line, i))
                    .unwrap()
        })
        .sum::<usize>()
        .to_string()
}
