use crate::input;

type Batteries = Vec<Vec<usize>>;

pub fn solve() -> (Option<usize>, Option<usize>) {
    let input = input::DAY_3_INPUT;
    let input = input.trim();

    let mut batteries: Batteries = Vec::new();

    for line in input.lines() {
        let mut bank = Vec::new();
        for c in line.chars() {
            match c {
                '0' => bank.push(0),
                '1' => bank.push(1),
                '2' => bank.push(2),
                '3' => bank.push(3),
                '4' => bank.push(4),
                '5' => bank.push(5),
                '6' => bank.push(6),
                '7' => bank.push(7),
                '8' => bank.push(8),
                '9' => bank.push(9),
                _ => {
                    unreachable!("Bad input data")
                }
            }
        }
        batteries.push(bank)
    }

    let part_1_total = solve_any_part(&batteries, 2);
    let part_2_total = solve_any_part(&batteries, 12);

    (part_1_total, part_2_total)
}

fn solve_any_part(batteries: &Batteries, number_of_digits: usize) -> Option<usize> {
    let mut total_jolts = 0;

    for bank in batteries {
        let mut digits_left = number_of_digits;
        let mut digits: Vec<usize> = Vec::with_capacity(digits_left);
        let mut start = 0;
        let mut end = bank.len() - digits_left;

        while digits_left > 0 {
            end += 1;
            let mut best = bank[start];
            let mut best_index = 0;

            for (i, &battery) in bank[start..end].iter().enumerate() {
                if battery > best {
                    best = battery;
                    best_index = i;
                }
            }

            start += best_index + 1;
            digits.push(best);
            digits_left -= 1;
        }

        let mut jolts = 0;
        for d in digits {
            jolts = jolts * 10 + d;
        }

        total_jolts += jolts;
    }

    Some(total_jolts)
}
