fn part1() {
    println!(
        "Part One: {}",
        include_bytes!("../input.txt")
            .split(|&b| b == b'\n')
            .filter(|line| !line.is_empty())
            .fold((50, 0), |(mut total, mut counter), x| {
                let direction = match x[0] {
                    b'L' => -1,
                    b'R' => 1,
                    _ => unreachable!(),
                };
                total += atoi::atoi::<isize>(&x[1..]).unwrap() * direction;
                if total % 100 == 0 {
                    counter += 1;
                }
                (total, counter)
            })
            .1
    )
}

fn part2() {
    println!(
        "Part Two: {}",
        include_bytes!("../input.txt")
            .split(|&b| b == b'\n')
            .filter(|line| !line.is_empty())
            .fold((50, 0), |(total, mut counter), x| {
                counter += atoi::atoi::<isize>(&x[1..]).unwrap() / 100;
                let direction = match x[0] {
                    b'L' => -1,
                    b'R' => 1,
                    _ => unreachable!(),
                };
                let n_total = total + (atoi::atoi::<isize>(&x[1..]).unwrap() % 100) * direction;

                if (n_total <= 0 && total != 0) || n_total > 99 {
                    counter += 1;
                }

                (n_total.rem_euclid(100), counter)
            })
            .1
    )
}

fn main() {
    part1();
    part2();
}
