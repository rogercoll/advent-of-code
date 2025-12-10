use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use ordered_float::OrderedFloat;

type Position = (isize, isize, isize);

fn part1() {
    let positions = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let nums = line
                .split(|b| *b == b',')
                .map(|num| atoi::atoi::<isize>(num).unwrap())
                .collect::<Vec<isize>>();
            (nums[0], nums[1], nums[2])
        })
        .collect::<Vec<Position>>();

    // ordered by distance
    let mut distances = BinaryHeap::<(Reverse<OrderedFloat<f64>>, (Position, Position))>::new();
    (0..positions.len()).for_each(|i| {
        (0..positions.len()).skip(i + 1).for_each(|j| {
            let distance = (((positions[i].0 - positions[j].0).pow(2)
                + (positions[i].1 - positions[j].1).pow(2)
                + (positions[i].2 - positions[j].2).pow(2)) as f64)
                .sqrt();
            distances.push((
                Reverse(OrderedFloat(distance)),
                (positions[i], positions[j]),
            ));
        });
    });

    let mut circuits = [0; 1000];
    let mut connections = HashMap::<Position, usize>::new();
    for circuit in 0..circuits.len() {
        let min = distances.pop().unwrap();
        let c_conn = connections.get_disjoint_mut([&min.1.0, &min.1.1]);
        match c_conn {
            [Some(circuit_a), Some(circuit_b)] => {
                if *circuit_a != *circuit_b {
                    let new_circuit = *circuit_a;
                    let prev_circuit = *circuit_b;
                    for (_, v) in connections.iter_mut() {
                        if *v == prev_circuit {
                            *v = new_circuit;
                            circuits[new_circuit] += 1;
                        }
                    }
                }
            }
            [Some(circuit), None] => {
                let circuit_id_val = *circuit;
                connections.insert(min.1.1, circuit_id_val);
                circuits[circuit_id_val] += 1;
            }
            [None, Some(circuit)] => {
                let circuit_id_val = *circuit;
                connections.insert(min.1.0, circuit_id_val);
                circuits[circuit_id_val] += 1;
            }
            [None, None] => {
                connections.insert(min.1.1, circuit);
                connections.insert(min.1.0, circuit);
                circuits[circuit] += 2;
            }
        }
    }
    circuits.sort_by_key(|w| Reverse(*w));
    println!("{}", circuits[0] * circuits[1] * circuits[2]);
}

fn part2() {
    let positions = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let nums = line
                .split(|b| *b == b',')
                .map(|num| atoi::atoi::<isize>(num).unwrap())
                .collect::<Vec<isize>>();
            (nums[0], nums[1], nums[2])
        })
        .collect::<Vec<Position>>();

    // ordered by distance
    let mut distances = BinaryHeap::<(Reverse<OrderedFloat<f64>>, (usize, usize))>::new();
    (0..positions.len()).for_each(|i| {
        (0..positions.len()).skip(i + 1).for_each(|j| {
            let distance = (((positions[i].0 - positions[j].0).pow(2)
                + (positions[i].1 - positions[j].1).pow(2)
                + (positions[i].2 - positions[j].2).pow(2)) as f64)
                .sqrt();
            distances.push((Reverse(OrderedFloat(distance)), (i, j)));
        });
    });

    let mut ds = aph_disjoint_set::DisjointSetArrayU16::<1000>::new();
    let mut last = (0, 0);
    while let Some((_, (a, b))) = distances.pop() {
        if matches!(ds.union(a, b), aph_disjoint_set::UnionResult::Success) {
            last = (a, b);
        }
    }
    println!("{}", positions[last.0].0 * positions[last.1].0);

    // let mut circuits = [0; 10000];
    // let mut connections = HashMap::<Position, usize>::new();
    // for circuit in 0..circuits.len() {
    //     let min = distances.pop().unwrap();
    //     let c_conn = connections.get_disjoint_mut([&min.1.0, &min.1.1]);
    //     let circuit_id = match c_conn {
    //         [Some(circuit_a), Some(circuit_b)] => {
    //             if *circuit_a != *circuit_b {
    //                 let new_circuit = *circuit_a;
    //                 let prev_circuit = *circuit_b;
    //                 for (_, v) in connections.iter_mut() {
    //                     if *v == prev_circuit {
    //                         *v = new_circuit;
    //                         circuits[new_circuit] += 1;
    //                     }
    //                 }
    //                 new_circuit
    //             } else {
    //                 *circuit_a
    //             }
    //         }
    //         [Some(circuit), None] => {
    //             let circuit_id_val = *circuit;
    //             connections.insert(min.1.1, circuit_id_val);
    //             circuits[circuit_id_val] += 1;
    //             circuit_id_val
    //         }
    //         [None, Some(circuit)] => {
    //             let circuit_id_val = *circuit;
    //             connections.insert(min.1.0, circuit_id_val);
    //             circuits[circuit_id_val] += 1;
    //             circuit_id_val
    //         }
    //         [None, None] => {
    //             connections.insert(min.1.1, circuit);
    //             connections.insert(min.1.0, circuit);
    //             circuits[circuit] += 2;
    //             circuit
    //         }
    //     };
    //     if circuits[circuit_id] == positions.len() {
    //         println!("{}", min.1.0.0 * min.1.1.0);
    //         return;
    //     }
    // }
}
fn main() {
    part1();
    part2();
}
