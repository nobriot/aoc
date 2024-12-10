use std::str::Split;

pub fn solve(input: &str) {
    // Problem 1
    let mut safe_count: usize = 0;

    for line in input.lines() {
        let numbers = line.trim().split(" ");
        if line_is_safe(&numbers, None) {
            safe_count += 1;
        }
    }

    println!("Part 1 Result: {safe_count}");

    // Problem 2
    // Just brute force here and pop all the numbers to see if we can
    // get the line to be safe
    let mut corrected_safe_count: usize = 0;

    'lines: for line in input.lines() {
        let numbers = line.trim().split(" ");
        if line_is_safe(&numbers, None) {
            // println!("Line is safe {line}");
            corrected_safe_count += 1;
            continue 'lines;
        }
        let elements = line.trim().split(" ").count();

        if elements < 2 {
            continue 'lines;
        }
        for pop_index in 0..elements {
            if line_is_safe(&numbers, Some(pop_index)) {
                // println!("Line is safe {line} with pop {pop_index}");
                corrected_safe_count += 1;
                continue 'lines;
            }
        }
    }
    println!("Part 2 Result: {corrected_safe_count}");
}

fn line_is_safe(numbers: &Split<'_, &str>, pop_index: Option<usize>) -> bool {
    let mut previous_number: Option<usize> = None;
    let mut increasing: Option<bool> = None;

    let count = numbers.clone().count();
    if count == 0 {
        return false;
    }
    let split = numbers.clone();
    for (i, n) in split.enumerate() {
        if let Some(pop) = pop_index {
            if pop == i {
                continue;
            }
        }
        // println!("parsing n {n}");
        let parse = n.parse::<usize>();
        if parse.is_err() {
            return false;
        }
        let next = parse.unwrap();

        match previous_number {
            None => previous_number = Some(next),
            Some(p) => {
                if p == next {
                    // println!("line: {line} is unsafe");
                    return false;
                } else {
                    let difference = p as isize - next as isize; // Guaranteed to be non-zero
                    if difference.abs() > 3 {
                        // println!("line: {line} is unsafe");
                        return false;
                    }

                    match increasing {
                        None => {
                            if difference < 0 {
                                increasing = Some(false);
                            } else {
                                increasing = Some(true);
                            }
                        }
                        Some(true) => {
                            if difference < 0 {
                                // println!("line: {line} is unsafe");
                                return false;
                            }
                        }
                        Some(false) => {
                            if difference > 0 {
                                // println!("line: {line} is unsafe");
                                return false;
                            }
                        }
                    }
                    previous_number = Some(next);
                }
            }
        }
    }
    // println!("line: {line} is safe");
    true
}
