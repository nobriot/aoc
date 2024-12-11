use crate::input;
use regex::Regex;

const NUMBERS_REGEX: &str = r#"(?<left>\d+) +(?<right>\d+)"#;

pub fn solve() -> (Option<usize>, Option<usize>) {
    let input = input::DAY_1_INPUT;
    let numbers_re = Regex::new(NUMBERS_REGEX).unwrap();

    let mut left_numbers: Vec<isize> = Vec::new();
    let mut right_numbers: Vec<isize> = Vec::new();

    // This is an ugly way to extract numbers:
    let captures = numbers_re.captures_iter(input);

    for value in captures {
        let left = value.name("left");
        let right = value.name("right");

        if left.is_none() || right.is_none() {
            eprintln!("Error parsing line");
            continue;
        }

        let left: isize = left.unwrap().as_str().parse::<isize>().unwrap();
        let right: isize = right.unwrap().as_str().parse::<isize>().unwrap();

        left_numbers.push(left);
        right_numbers.push(right);
    }

    left_numbers.sort();
    right_numbers.sort();

    debug_assert!(left_numbers.len() == right_numbers.len());

    let mut result: isize = 0;
    for i in 0..left_numbers.len() {
        result += (left_numbers[i] - right_numbers[i]).abs();
    }

    // println!("Part 1 Result: {result}");

    // Similarity score
    let mut similarity_score: usize = 0;

    for value in left_numbers {
        similarity_score += value as usize * right_numbers.iter().filter(|&n| *n == value).count();
    }
    // println!("Part 2 Result: {similarity_score}");

    (Some(result as usize), Some(similarity_score as usize))
}
