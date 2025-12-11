use crate::input;

pub fn solve() -> (Option<usize>, Option<usize>) {
    let input = input::DAY_2_INPUT;
    let input = input.trim();

    let mut ranges: Vec<(usize, usize)> = Vec::new();

    for range in input.split(",") {
        if let Some((start, end)) = range.split_once('-') {
            let start_usize = start.parse::<usize>().unwrap();
            let end_usize = end.parse::<usize>().unwrap();

            ranges.push((start_usize, end_usize))
        }
    }

    let part_1_total = solve_part_1(&ranges);
    let part_2_total = solve_part_2(&ranges);

    (part_1_total, part_2_total)
}

fn solve_part_1(ranges: &Vec<(usize, usize)>) -> Option<usize> {
    let mut invalid_ids = 0;
    for &(start, stop) in ranges {
        for i in start..=stop {
            // Get the number of digits
            let digits = count_digits(i);
            if !digits.is_multiple_of(2) {
                continue;
            }
            // Find the middle
            let half = (digits / 2) as usize;
            let divider = 10_usize.pow(half as u32);
            let high_digits = i / divider;
            let low_digits = i % divider;

            if high_digits == low_digits {
                invalid_ids += i
            }
        }
    }

    Some(invalid_ids)
}

fn solve_part_2(ranges: &Vec<(usize, usize)>) -> Option<usize> {
    // Hmm is there something better here than find all possible divisors of the number
    // of digits and check them all ?
    //
    let mut invalid_ids = 0;

    for &(start, stop) in ranges {
        'number: for i in start..=stop {
            // Get the number of digits
            let digits = count_digits(i);

            'digits: for d in 1..=(digits / 2) {
                if !digits.is_multiple_of(d) {
                    continue;
                }

                let divider = 10_usize.pow(d);
                let low_digits = i % divider;
                let groups = digits / d;

                let mut current_divider = divider;
                for _ in 0..(groups - 1) {
                    let digits_group = (i / current_divider) % divider;
                    current_divider *= 10_usize.pow(d);

                    if digits_group != low_digits {
                        continue 'digits;
                    }
                }

                // If we got here without hitting one of the continues, all subgroups are equal
                invalid_ids += i;
                continue 'number;
            }
        }
    }

    Some(invalid_ids)
}

fn count_digits(number: usize) -> u32 {
    if number == 0 {
        return 1;
    }
    number.ilog10() + 1
}
