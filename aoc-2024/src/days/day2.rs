use itertools::Itertools;

pub(crate) fn part1() {
    println!(
        "{}",
        include_bytes!("../../input/day2.txt")
            .split(|&b| b == b'\n')
            .filter(|line| {
                !line.is_empty() && {
                    line.split(|&b| b == b' ')
                        .map(|x| atoi::atoi::<isize>(x).unwrap())
                        .tuple_windows()
                        .try_fold(0, |ord, (a, b)| {
                            let diff = b - a;
                            if ord >= 0 && (diff <= 3) && (diff >= 1) {
                                Ok(1)
                            } else if ord <= 0 && (diff >= -3) && (diff <= -1) {
                                Ok(-1)
                            } else {
                                Err(())
                            }
                        })
                        .is_ok()
                }
            })
            .count()
    )
}

pub(crate) fn part2() {
    println!(
        "{}",
        include_bytes!("../../input/day2.txt")
            .split(|&b| b == b'\n')
            .filter(|line| !line.is_empty())
            .filter(|line| {
                let nums = line
                    .split(|&b| b == b' ')
                    .map(|n| atoi::atoi::<isize>(n).unwrap())
                    .collect::<Vec<_>>();
                (0..nums.len()).any(|i| {
                    nums[0..i]
                        .iter()
                        .chain(&nums[i + 1..])
                        .tuple_windows()
                        .try_fold(0, |ord, (a, b)| {
                            if ord >= 0 && (1..=3).contains(&(b - a)) {
                                Ok(1)
                            } else if ord <= 0 && (1..=3).contains(&(a - b)) {
                                Ok(-1)
                            } else {
                                Err(())
                            }
                        })
                        .is_ok()
                })
            })
            .count(),
    );
}
