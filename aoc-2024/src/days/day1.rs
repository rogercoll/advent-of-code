use std::collections::HashMap;

pub(crate) fn part1() {
    let num_len = include_bytes!("../../input/day1.txt")
        .iter()
        .position(|&b| b == b' ')
        .unwrap();

    let (mut a, mut b): (Vec<usize>, Vec<usize>) = include_bytes!("../../input/day1.txt")
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            println!("{:?}", atoi::atoi::<usize>(&line[0..num_len]).unwrap());
            (
                atoi::atoi::<usize>(&line[0..num_len]).unwrap(),
                atoi::atoi::<usize>(&line[num_len + 3..]).unwrap(),
            )
        })
        .unzip();

    a.sort_unstable();
    b.sort_unstable();

    println!(
        "{}",
        a.iter().zip(b).map(|(a, b)| a.abs_diff(b)).sum::<usize>()
    )
}

pub(crate) fn part2() {
    let num_len = include_bytes!("../../input/day1.txt")
        .iter()
        .position(|&b| b == b' ')
        .unwrap();

    let (mut a, mut b) = (Vec::with_capacity(1000), HashMap::new());

    for line in include_bytes!("../../input/day1.txt")
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
    {
        a.push(atoi::atoi::<usize>(&line[0..num_len]).unwrap());
        b.entry(atoi::atoi::<usize>(&line[num_len + 3..]).unwrap())
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    println!(
        "{}",
        a.iter()
            .map(|val_a| b.get(val_a).unwrap_or(&0) * val_a)
            .sum::<usize>()
    )
}
