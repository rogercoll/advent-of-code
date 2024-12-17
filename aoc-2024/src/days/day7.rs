use indexmap::Equivalent;

pub(crate) fn part1() {
    println!(
        "{}",
        include_str!("../../input/day7.txt")
            .split('\n')
            .filter_map(|line| {
                if line.is_empty() {
                    return None;
                } else {
                    let eq = line.split_once(":").unwrap();
                    return Some((
                        eq.0.parse::<u64>().unwrap(),
                        eq.1.split_whitespace()
                            .map(|num| num.parse::<u64>().unwrap())
                            .collect::<Vec<u64>>(),
                    ));
                }
            })
            .filter(|(result, values)| {
                (0..2_u32.pow(u32::try_from(values.len() - 1).unwrap())).any(|mut i| {
                    values
                        .iter()
                        .copied()
                        .reduce(|acc, e| {
                            // println!("{i:b}");
                            if i & 1 == 0 {
                                i >>= 1;
                                acc + e
                            } else {
                                i >>= 1;
                                acc * e
                            }
                        })
                        .unwrap()
                        .equivalent(result)
                })
            })
            .map(|(result, _values)| result)
            .sum::<u64>()
    )
}

pub(crate) fn part2() {
    println!(
        "{}",
        include_str!("../../input/day7.txt")
            .split('\n')
            .filter_map(|line| {
                if line.is_empty() {
                    return None;
                } else {
                    let eq = line.split_once(":").unwrap();
                    return Some((
                        eq.0.parse::<u64>().unwrap(),
                        eq.1.split_whitespace()
                            .map(|num| num.parse::<u64>().unwrap())
                            .collect::<Vec<u64>>(),
                    ));
                }
            })
            .filter(|(result, values)| {
                (0..3_u32.pow(u32::try_from(values.len() - 1).unwrap())).any(|mut i| {
                    values
                        .iter()
                        .copied()
                        .reduce(|mut acc, e| {
                            if i % 3 == 0 {
                                i /= 3;
                                acc + e
                            } else if i % 3 == 1 {
                                i /= 3;
                                acc * e
                            } else {
                                i /= 3;
                                acc *= 10_u64.pow(e.ilog10() + 1);
                                acc + e
                            }
                        })
                        .unwrap()
                        .equivalent(result)
                })
            })
            .map(|(result, _values)| result)
            .sum::<u64>()
    )
}
