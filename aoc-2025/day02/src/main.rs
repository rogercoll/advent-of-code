#![feature(slice_split_once)]

fn part1() {
    println!(
        "{}",
        include_bytes!("../input.txt")
            .split(|&b| b == b',')
            .filter(|range| !range.is_empty())
            .filter_map(|range| {
                let (init, end) = range.split_once(|b| *b == b'-').unwrap();
                let mut repeated_digits = vec![];
                for val in atoi::atoi::<isize>(init).unwrap()..=atoi::atoi::<isize>(end).unwrap() {
                    let c_val = val.to_string();
                    let c_val_len = c_val.len();
                    if c_val_len % 2 == 0 && c_val[0..c_val_len / 2] == c_val[c_val_len / 2..] {
                        repeated_digits.push(val);
                    }
                }
                if repeated_digits.len() > 0 {
                    return Some(repeated_digits);
                }
                None
            })
            .flatten()
            .sum::<isize>()
    )
}

fn part2() {
    println!(
        "{}",
        include_bytes!("../input.txt")
            .split(|&b| b == b',')
            .filter(|range| !range.is_empty())
            .filter_map(|range| {
                let (init, end) = range.split_once(|b| *b == b'-').unwrap();
                let mut repeated_digits = vec![];
                for val in atoi::atoi::<isize>(init).unwrap()..=atoi::atoi::<isize>(end).unwrap() {
                    let s_val = val.to_string();
                    let (mut prefix_l, mut rp) = (1, 1);
                    while rp + prefix_l <= s_val.len() {
                        if s_val[0..prefix_l] == s_val[rp..rp + prefix_l] {
                            rp += prefix_l;
                            if rp + prefix_l > s_val.len() {
                                if s_val.matches(&s_val[0..prefix_l]).count() * prefix_l
                                    == s_val.len()
                                {
                                    repeated_digits.push(val);
                                }
                            }
                        } else {
                            prefix_l += 1;
                            rp = prefix_l;
                        }
                    }
                }
                if repeated_digits.len() > 0 {
                    return Some(repeated_digits);
                }
                None
            })
            .flatten()
            .sum::<isize>()
    )
}

fn main() {
    // part1();
    part2();
}
