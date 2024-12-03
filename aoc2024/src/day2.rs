use std::fs;

pub fn solve(input_file: &str) {
    // Parse the file
    let contents = fs::read_to_string(input_file).expect("Unable to read the file");

    // Problem 1
    let mut safe_count: usize = 0;

    'outter: for line in contents.lines() {
        let numbers = line.trim().split(" ");
        let mut previous_number: Option<usize> = None;
        let mut increasing: Option<bool> = None;

        for n in numbers {
            // println!("parsing n {n}");
            if n.trim().is_empty() {
                continue 'outter;
            }
            let next = n.parse::<usize>().unwrap();

            match previous_number {
                None => previous_number = Some(next),
                Some(p) => {
                    if p == next {
                        println!("line: {line} is unsafe");
                        continue 'outter;
                    } else {
                        let difference = p as isize - next as isize; // Guaranteed to be non-zero
                        if difference.abs() > 3 {
                            println!("line: {line} is unsafe");
                            continue 'outter;
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
                                    println!("line: {line} is unsafe");
                                    continue 'outter;
                                }
                            }
                            Some(false) => {
                                if difference > 0 {
                                    println!("line: {line} is unsafe");
                                    continue 'outter;
                                }
                            }
                        }
                        previous_number = Some(next);
                    }
                }
            }
        }
        println!("line: {line} is safe");
        safe_count += 1;
    }

    println!("Part 1 Result: {safe_count}");

    // // Similarity score
    // let mut similarity_score: usize = 0;

    // for value in left_numbers {
    //     similarity_score += value as usize * right_numbers.iter().filter(|&n| *n == value).count();
    // }
    // println!("Part 2 Result: {similarity_score}");
}
