// use std::collections::HashMap;

use crate::input;

const PART_1_CONNECTIONS: usize = 1000;

pub fn solve() -> (Option<usize>, Option<usize>) {
    let input = input::DAY_8_INPUT;

    // Parse input into boxes
    let mut boxes: Vec<(usize, usize, usize)> = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut split = line.split(',');
        let x = split
            .next()
            .expect("Valid x coordinate")
            .parse::<usize>()
            .expect("Valid digit");
        let y = split
            .next()
            .expect("Valid y coordinate")
            .parse::<usize>()
            .expect("Valid digit");
        let z = split
            .next()
            .expect("Valid z coordinate")
            .parse::<usize>()
            .expect("Valid digit");

        boxes.push((x, y, z));
    }

    // Compute the distances of all pairs
    let mut distances = Vec::new();

    for (i, r#box) in boxes.iter().enumerate() {
        for (j, box2) in boxes.iter().enumerate().skip(i + 1) {
            let distance = calculate_distance(box2, r#box);

            distances.push((i, j, distance));
        }
    }

    // Sort by smallest distance
    distances.sort_unstable_by_key(|(_, _, d)| *d);

    // eprintln!( "Distance 1 : {:?} -> {:?} = {:?}", boxes[distances[0].0], boxes[distances[0].1], distances[0].2 );
    // eprintln!( "Distance 2 : {:?} -> {:?} = {:?}", boxes[distances[1].0], boxes[distances[1].1], distances[1].2 );
    // eprintln!( "Distance 3 : {:?} -> {:?} = {:?}", boxes[distances[2].0], boxes[distances[2].1], distances[2].2 );

    // Create a vec containing all the circuits
    let mut circuits: Vec<Vec<usize>> = Vec::with_capacity(boxes.len());
    for (i, _) in boxes.iter().enumerate() {
        circuits.push(vec![i]);
    }

    // Start connecting boxes and reducing circuits
    for &(i, j, _) in distances.iter().take(PART_1_CONNECTIONS) {
        let mut i_c = None;
        let mut j_c = None;

        for (ci, c) in circuits.iter().enumerate() {
            if c.contains(&i) {
                i_c = Some(ci);
                if j_c.is_some() {
                    break;
                }
            }
            if c.contains(&j) {
                j_c = Some(ci);
                if i_c.is_some() {
                    break;
                }
            }
        }

        match (i_c, j_c) {
            (None, None) => {
                let new_circuit = vec![i, j];
                circuits.push(new_circuit);
            }
            (None, Some(x)) => {
                circuits[x].push(i);
            }
            (Some(x), None) => {
                circuits[x].push(j);
            }
            (Some(x), Some(y)) => {
                if x == y {
                    // Already in the same circuit, nothing to do
                    continue;
                }
                let first = x.min(y);
                let second = x.max(y);

                let mut ci = circuits.swap_remove(second);
                let cj = circuits.swap_remove(first);

                ci.extend(cj);
                circuits.push(ci);
            }
        }
    }

    // Find out the 3 largest circuits:
    circuits.sort_unstable_by_key(|v| -(v.len() as isize));
    let part_1_total = circuits[0].len() * circuits[1].len() * circuits[2].len();

    // Go at it again, until we have a single circuit
    let mut part_2_total = None;
    for &(i, j, _) in distances.iter().skip(PART_1_CONNECTIONS) {
        let mut i_c = None;
        let mut j_c = None;

        for (ci, c) in circuits.iter().enumerate() {
            if c.contains(&i) {
                i_c = Some(ci);
                if j_c.is_some() {
                    break;
                }
            }
            if c.contains(&j) {
                j_c = Some(ci);
                if i_c.is_some() {
                    break;
                }
            }
        }

        match (i_c, j_c) {
            (None, None) => {
                let new_circuit = vec![i, j];
                circuits.push(new_circuit);
            }
            (None, Some(x)) => {
                circuits[x].push(i);
            }
            (Some(x), None) => {
                circuits[x].push(j);
            }
            (Some(x), Some(y)) => {
                if x == y {
                    // Already in the same circuit, nothing to do
                    continue;
                }
                let first = x.min(y);
                let second = x.max(y);

                let mut ci = circuits.swap_remove(second);
                let cj = circuits.swap_remove(first);

                ci.extend(cj);
                circuits.push(ci);
            }
        }
        if circuits.len() == 1 {
            // We're done. Compute the value and break
            part_2_total = Some(boxes[i].0 * boxes[j].0);
            break;
        }
    }

    (Some(part_1_total), part_2_total)
}

/// We don't really need to square root, as this function is monotonic and will
/// give us the same ordering as if we don't apply it.
fn calculate_distance(lhs: &(usize, usize, usize), rhs: &(usize, usize, usize)) -> usize {
    let x = lhs.0 as isize - rhs.0 as isize;
    let y = lhs.1 as isize - rhs.1 as isize;
    let z = lhs.2 as isize - rhs.2 as isize;

    (x.pow(2) + y.pow(2) + z.pow(2)) as usize
}
