use regex::Regex;
use std::fs;

const MUL_REGEX: &str = r#"mul\((?<X>\d{1,3}),(?<Y>\d+{1,3})\)"#;

enum Enablement {
    Do,
    Dont,
}

pub fn solve(input_file: &str) {
    // Parse the file
    let contents = fs::read_to_string(input_file).expect("Unable to read the file");

    let part_1_total = compute_multiplications(&contents);
    println!("Part 1 Result: {part_1_total}");

    let part_2_total: usize = compute_enabled_multiplications(&contents);
    println!("Part 2 Result: {part_2_total}");
}

fn compute_enabled_multiplications(input: &str) -> usize {
    let mut total: usize = 0;

    let mut e = Enablement::Do;
    let mut start: usize = 0;
    let mut stop: usize = input.len();

    // println!("Total length {stop}");
    // println!("Test stlice {}", &input[4..10]);
    while start < input.len() {
        // println!("Iteration with {start}");
        match e {
            Enablement::Do => {
                let pos = &input[start..].find("don't()");
                if let Some(p) = pos {
                    e = Enablement::Dont;
                    stop = *p;
                }
                // println!("Running from {start} to {}", start + stop);
                total += compute_multiplications(&input[start..(start + stop)]);
                start = start + stop;
                stop = input.len() - start;
            }
            Enablement::Dont => {
                let pos = &input[start..].find("do()");
                if let Some(p) = pos {
                    e = Enablement::Do;
                    stop = *p;
                }
                // println!("Skipping from {start} to {}", start + stop);
                start = start + stop;
                stop = input.len() - start;
            }
        }
    }

    total
}

fn compute_multiplications(input: &str) -> usize {
    let mut total: usize = 0;

    let mul_re = Regex::new(MUL_REGEX).unwrap();
    let captures = mul_re.captures_iter(&input);

    for c in captures {
        let x = c.name("X");
        let y = c.name("Y");

        if x.is_none() || y.is_none() {
            eprintln!("Error parsing line {:?}", c);
            continue;
        }

        let x = x.unwrap().as_str().parse::<usize>().unwrap();
        let y = y.unwrap().as_str().parse::<usize>().unwrap();

        total += x * y;
    }

    total
}
