use std::collections::HashMap;

pub(crate) fn part1() {
    let mut fid = 0;
    let mut disk: Vec<isize> = include_str!("../../input/day9.txt")
        .trim_end()
        .chars()
        .enumerate()
        .fold(Vec::new(), |mut acc, (i, value)| {
            let value = value.to_digit(10).unwrap() as usize;
            if i % 2 == 0 {
                acc.extend(vec![fid; value]);
                fid += 1;
            } else {
                acc.extend(vec![-1; value])
            }
            acc
        });

    let free_spaces = disk
        .iter()
        .enumerate()
        .filter_map(|(i, val)| {
            if *val == -1 {
                return Some(i);
            }
            return None;
        })
        .collect::<Vec<usize>>();

    for i in free_spaces {
        while *disk.last().unwrap() == -1 {
            disk.pop();
        }
        if disk.len() <= i {
            break;
        }
        disk[i] = disk.pop().unwrap();
    }

    println!(
        "{:?}",
        disk.iter()
            .enumerate()
            .map(|(i, val)| i as isize * *val)
            .sum::<isize>()
    );
}

pub(crate) fn part2() {
    let mut fid = 0;
    let mut pos = 0;

    let (mut files, mut blanks) = include_str!("../../input/day9.txt")
        .trim_end()
        .chars()
        .enumerate()
        .fold(
            (HashMap::new(), Vec::new()),
            |(mut files, mut blanks), (i, value)| {
                let value = value.to_digit(10).unwrap() as usize;
                if i % 2 == 0 {
                    files.insert(fid, (pos, value));
                    fid += 1;
                } else if value != 0 {
                    blanks.push((pos, value));
                }
                pos += value;
                (files, blanks)
            },
        );

    while fid > 0 {
        fid -= 1;
        let (fpos, size) = files.get(&fid).unwrap().clone();
        for (i, (start, length)) in blanks.iter().enumerate() {
            if *start >= fpos {
                blanks = blanks[0..i].to_vec();
                break;
            } else if size <= *length {
                files.insert(fid, (*start, size));
                if size == *length {
                    blanks.remove(i);
                } else {
                    blanks[i] = (start + size, length - size);
                }
                break;
            }
        }
    }

    println!(
        "{:?}",
        files
            .iter()
            .map(|(fid, (pos, size))| (*pos..(pos + size)).map(|val| val * fid).sum::<usize>())
            .sum::<usize>()
    );
}
