use nalgebra::{Matrix2, Vector2};

pub struct EqSystem {
    incog: Matrix2<f64>,
    prize: Vector2<f64>,
}

pub(crate) fn part1() {
    let machines = include_str!("../../input/day13.txt")
        .split("\n\n")
        .map(|machine| {
            let data = machine
                .split("\n")
                .filter(|line| !line.is_empty())
                .collect::<Vec<&str>>();
            let extract = |line: &str| -> (f64, f64) {
                let parts: Vec<&str> = line.split(&[',', ' '][..]).collect();
                let x_part = parts.iter().find(|&&part| part.starts_with("X")).unwrap();
                let y_part = parts.iter().find(|&&part| part.starts_with("Y")).unwrap();

                let x_val = x_part[2..].parse::<f64>().ok().unwrap(); // Remove "X+" or "X=" and parse
                let y_val = y_part[2..].parse::<f64>().ok().unwrap(); // Remove "Y+" or "Y=" and parse
                (x_val, y_val)
            };
            let a = extract(data[0]);
            let b = extract(data[1]);
            let prize = extract(data[2]);
            EqSystem {
                incog: Matrix2::new(a.0, b.0, a.1, b.1),
                prize: Vector2::new(prize.0, prize.1),
            }
        })
        .flat_map(|sys| Some(sys.incog.try_inverse()? * sys.prize))
        .filter(|s| s.iter().all(|c| (c - c.round()).abs() < 0.001))
        .map(|s| s.x.round() as usize * 3 + s.y.round() as usize)
        // .inspect(|s| println!("Tokens: {s:?}"))
        .sum::<usize>();

    println!("{:?}", machines)
}

pub(crate) fn part2() {
    let machines = include_str!("../../input/day13.txt")
        .split("\n\n")
        .map(|machine| {
            let data = machine
                .split("\n")
                .filter(|line| !line.is_empty())
                .collect::<Vec<&str>>();
            let extract = |line: &str| -> (f64, f64) {
                let parts: Vec<&str> = line.split(&[',', ' '][..]).collect();
                let x_part = parts.iter().find(|&&part| part.starts_with("X")).unwrap();
                let y_part = parts.iter().find(|&&part| part.starts_with("Y")).unwrap();

                let x_val = x_part[2..].parse::<f64>().ok().unwrap(); // Remove "X+" or "X=" and parse
                let y_val = y_part[2..].parse::<f64>().ok().unwrap(); // Remove "Y+" or "Y=" and parse
                (x_val, y_val)
            };
            let a = extract(data[0]);
            let b = extract(data[1]);
            let prize = extract(data[2]);
            EqSystem {
                incog: Matrix2::new(a.0, b.0, a.1, b.1),
                prize: Vector2::new(prize.0 + 10000000000000.0, prize.1 + 10000000000000.0),
            }
        })
        .flat_map(|sys| Some(sys.incog.try_inverse()? * sys.prize))
        .filter(|s| s.iter().all(|c| (c - c.round()).abs() < 0.001))
        .map(|s| s.x.round() as usize * 3 + s.y.round() as usize)
        // .inspect(|s| println!("Tokens: {s:?}"))
        .sum::<usize>();

    println!("{:?}", machines)
}
