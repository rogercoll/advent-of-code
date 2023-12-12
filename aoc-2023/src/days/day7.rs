pub fn part1(input: String) -> String {
    let mut hands: Vec<_> = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();

            let (mut strength, high_card) = hand.chars().enumerate().fold(
                ([0; 13], 0),
                |(mut strength, mut high_card), (i, card)| {
                    let card: usize = match card {
                        'A' => 12,
                        'K' => 11,
                        'Q' => 10,
                        'J' => 9,
                        'T' => 8,
                        d => d.to_digit(10).unwrap() as usize - 2,
                    };
                    strength[card] += 1;
                    high_card |= (card as u32) << 4 * (4 - i);
                    (strength, high_card)
                },
            );

            strength.sort();
            let strength = match strength[strength.len() - 1] {
                5 => 6,
                4 => 5,
                3 if strength[strength.len() - 1] == 2 => 4,
                3 => 3,
                2 if strength[strength.len() - 1] == 2 => 2,
                2 => 1,
                _ => 0,
            };

            (high_card | (strength << 29), bid.parse::<u32>().unwrap())
        })
        .collect();

    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(i, vals)| vals.1 * (i as u32 + 1))
        .sum::<u32>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let mut hands: Vec<_> = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();

            let (mut strength, high_card, jokers) = hand.chars().enumerate().fold(
                ([0; 13], 0, 0),
                |(mut strength, mut high_card, mut jokers), (i, card)| {
                    let card: usize = match card {
                        'A' => 12,
                        'K' => 11,
                        'Q' => 10,
                        'T' => 9,
                        'J' => {
                            jokers += 1;
                            0
                        }
                        d => d.to_digit(10).unwrap() as usize - 1,
                    };
                    // do not count joker strength
                    strength[card] += 1 * (card != 0) as usize;
                    high_card |= (card as u32) << 4 * (4 - i);
                    (strength, high_card, jokers)
                },
            );

            // ascending order
            strength.sort();
            let strength = match strength[strength.len() - 1] + jokers {
                5 => 6,
                4 => 5,
                3 if strength[strength.len() - 2] == 2 => 4,
                3 => 3,
                2 if strength[strength.len() - 2] == 2 => 2,
                2 => 1,
                _ => 0,
            };

            (high_card | (strength << 29), bid.parse::<u32>().unwrap())
        })
        .collect();

    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(i, vals)| vals.1 * (i as u32 + 1))
        .sum::<u32>()
        .to_string()
}
