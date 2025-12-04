use atoi::atoi;

fn part1() {
    println!(
        "Part 1: {}",
        include_bytes!("../input.txt")
            .split(|&b| b == b'\n')
            .filter(|line| !line.is_empty())
            .map(|line| {
                let (mut l, mut maxl, mut maxr) = (0, 0, 0);
                while l < line.len() - 1 {
                    if line[l] > line[maxl] || l == 0 {
                        maxl = l;
                        let (mut r, mut maxrr) = (l + 1, l + 1);
                        while r < line.len() {
                            if line[r] > line[maxrr] {
                                maxrr = r;
                            }
                            r += 1;
                        }
                        maxr = maxrr;
                    }
                    l += 1;
                }
                atoi::<i32>(&[line[maxl], line[maxr]]).unwrap()
            })
            .sum::<i32>()
    )
}

fn part2() {
    println!(
        "Part 2: {}",
        include_bytes!("../input.txt")
            .split(|&b| b == b'\n')
            .filter(|line| !line.is_empty())
            .map(|mut batteries| {
                let mut remaining = 12;
                let mut result = vec![];
                while remaining > 0 {
                    let left_end_idx = batteries.len() - remaining + 1;
                    if left_end_idx == 0 {
                        result.extend_from_slice(batteries);
                        break;
                    }
                    let (pos, &max_val) = batteries[0..left_end_idx]
                        .iter()
                        .enumerate()
                        .rev()
                        .max_by_key(|&(_idx, val)| val)
                        .unwrap();
                    result.push(max_val);
                    remaining -= 1;
                    batteries = &batteries[pos + 1..];
                }
                atoi::<u64>(result.as_slice()).unwrap()
            })
            .sum::<u64>()
    )
}

fn main() {
    part1();
    part2();
}
