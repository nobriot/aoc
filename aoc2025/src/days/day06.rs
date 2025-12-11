use crate::input;

pub fn solve() -> (Option<usize>, Option<usize>) {
    let input = input::DAY_6_INPUT;

    let mut numbers = Vec::new();
    let mut operands = Vec::new();

    for (i, line) in input.lines().enumerate() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        for (col, token) in line.split_whitespace().enumerate() {
            if i == 0 {
                numbers.push(Vec::new());
            }
            match token.chars().next() {
                Some(c) if c == '+' || c == '*' => {
                    operands.push(c);
                }
                _ => {
                    let number = token.parse::<usize>().expect("correct input");
                    numbers[col].push(number)
                }
            }
        }
    }

    assert_eq!(operands.len(), numbers.len());

    let part_1_total = solve_part_1(&numbers, &operands);
    let part_2_total = solve_part_2(input, &operands);

    (part_1_total, part_2_total)
}

fn solve_part_1(numbers: &[Vec<usize>], operands: &[char]) -> Option<usize> {
    let mut total = 0;

    for (i, nums) in numbers.iter().enumerate() {
        let operand = operands[i];

        let mut local_sum = match operand {
            '+' => 0,
            '*' => 1,
            c => panic!("Bad operand: {:?}", c),
        };

        for &n in nums {
            match operand {
                '+' => local_sum += n,
                '*' => local_sum *= n,
                c => panic!("Bad operand: {:?}", c),
            };
        }

        total += local_sum;
    }

    Some(total)
}

fn solve_part_2(input: &str, operands: &[char]) -> Option<usize> {
    let mut total = 0;
    let mut col_offset: usize = 0;

    for operand in operands.iter() {
        let mut local_sum = match operand {
            '+' => 0,
            '*' => 1,
            c => panic!("Bad operand: {:?}", c),
        };
        loop {
            let mut vertical_number = String::new();
            for line in input.lines() {
                if line.contains('+') {
                    continue;
                }
                let line = line.as_bytes();
                if col_offset >= line.len() {
                    break;
                }
                match line[col_offset] {
                    c if c.is_ascii_digit() => vertical_number.push(char::from(c)),
                    _ => {}
                }
            }

            if vertical_number.is_empty() {
                col_offset += 1;
                break;
            }

            let number = vertical_number.parse::<usize>().expect("correct number");

            match operand {
                '+' => local_sum += number,
                '*' => local_sum *= number,
                c => panic!("Bad operand: {:?}", c),
            };
            col_offset += 1;
        }

        // Find highest digit in the collection
        total += local_sum;
    }
    Some(total)
}
