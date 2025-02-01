use crate::input;
use itertools::Itertools;

pub fn solve() -> (Option<usize>, Option<usize>) {
    let input = input::DAY_21_INPUT;
    let codes = Codes::from_str(input);

    let part_1_total = solve_part_1(&codes);
    let part_2_total = solve_part_2(&codes);

    (part_1_total, part_2_total)
}

fn solve_part_1(codes: &Codes) -> Option<usize> {
    Some(codes.solve())
}

fn solve_part_2(codes: &Codes) -> Option<usize> {
    None
    //Some(codes.solve_part_2())
}

#[derive(Debug)]
pub struct Codes<'a> {
    pub codes: Vec<&'a str>,
}

impl<'a> Codes<'a> {
    const NUM_KEYPAD: [&'a [char]; 4] = [
        &['7', '8', '9'],
        &['4', '5', '6'],
        &['1', '2', '3'],
        &[' ', '0', 'A'],
    ];

    const ARROW_KEYPAD: [&'a [char]; 2] = [&[' ', '^', 'A'], &['<', 'v', '>']];

    pub fn from_str(input: &'a str) -> Self {
        let codes = input.lines().collect();
        Self { codes }
    }

    fn at(keypad: &[&[char]], line: isize, pos: isize) -> Option<char> {
        if line < 0 || pos < 0 {
            return None;
        }
        let line = line as usize;
        let pos = pos as usize;
        keypad.get(line)?.get(pos).copied()
    }

    fn find_char(keypad: &[&[char]], c: char) -> Option<(isize, isize)> {
        for (l, line) in keypad.iter().enumerate() {
            for (p, ch) in line.iter().enumerate() {
                if *ch == c {
                    return Some((l as isize, p as isize));
                }
            }
        }
        None
    }

    pub fn keep_shortest(codes: &mut Vec<String>) {
        let mut shortest = 0;
        let mut shortest_length = codes[0].len();
        codes.iter().enumerate().for_each(|(i, a)| {
            if a.len() < shortest_length {
                shortest = i;
                shortest_length = a.len();
            }
        });

        codes.retain(|c| c.len() == shortest_length);
    }

    pub fn solve(&self) -> usize {
        let mut total = 0;
        for code in &self.codes {
            println!("-----------------------------------");
            println!("Solving Code: {}", code);
            let first_codes = Self::solve_keypad(&Self::NUM_KEYPAD, code);

            let mut final_codes = Self::iterate_keypad(&Self::ARROW_KEYPAD, first_codes, 2);

            // Update the total
            for s in &final_codes {
                // println!("Solution: {}", s);
            }
            println!("number of solutions : {}", final_codes.len());
            let solution = final_codes.swap_remove(0);
            let mut code_num = String::from(*code);
            code_num = code_num.replace("A", "");
            let code_num = code_num.parse::<usize>().unwrap();
            println!("Solution: {} - {}", code_num, solution.len());

            total += code_num * solution.len();
        }

        total
    }

    pub fn solve_part_2(&self) -> usize {
        let mut total = 0;
        for code in &self.codes {
            println!("-----------------------------------");
            println!("Solving Code: {}", code);
            let first_codes = Self::solve_keypad(&Self::NUM_KEYPAD, code);

            let mut final_codes = Self::iterate_keypad(&Self::ARROW_KEYPAD, first_codes, 25);

            // Update the total
            println!("number of solutions : {}", final_codes.len());
            let solution = final_codes.swap_remove(0);
            let mut code_num = String::from(*code);
            code_num = code_num.replace("A", "");
            let code_num = code_num.parse::<usize>().unwrap();
            println!("Solution: {} - {}", code_num, solution.len());

            total += code_num * solution.len();
        }

        total
    }

    pub fn iterate_keypad(keypad: &[&[char]], codes: Vec<String>, i: usize) -> Vec<String> {
        println!(
            "Iterating keypad: {} - size of the codes {}",
            i,
            codes.len()
        );

        if i == 0 {
            return codes;
        }
        let mut new_codes: Vec<String> = Vec::new();
        codes.iter().for_each(|c| {
            new_codes.append(&mut Self::solve_keypad(keypad, c));
        });
        Self::keep_shortest(&mut new_codes);
        new_codes.drain(1..);
        new_codes = Self::iterate_keypad(keypad, new_codes, i - 1);
        new_codes
    }

    pub fn solve_keypad(keypad: &[&[char]], keys: &str) -> Vec<String> {
        let mut position: (isize, isize) = Self::find_char(keypad, 'A').unwrap();
        let mut combinations: Vec<String> = Vec::new();

        for key in keys.chars() {
            //println!("key: {}", key);
            let target = Self::find_char(keypad, key).unwrap();
            // println!("target: {:?}", target);
            let paths = Self::generate_paths(keypad, position, target);
            // println!("paths: {:?}", paths);
            let mut new_combinations = Vec::new();

            if combinations.is_empty() {
                for path in &paths {
                    new_combinations.push(format!("{}A", path.iter().collect::<String>()));
                }
            } else {
                for combination in combinations {
                    for path in &paths {
                        new_combinations.push(format!(
                            "{}{}A",
                            combination,
                            path.iter().collect::<String>()
                        ));
                    }
                }
            }
            //println!("New combinations: {:?}", new_combinations);
            position = target;
            combinations = new_combinations;
        }

        combinations
    }

    pub fn generate_paths(
        keypad: &[&[char]],
        from: (isize, isize),
        to: (isize, isize),
    ) -> Vec<Vec<char>> {
        let position = from;
        let target = to;

        let mut items: Vec<String> = Vec::new();
        if target.0 < position.0 {
            items.push("^".repeat((position.0 - target.0) as usize));
        }
        if target.1 > position.1 {
            items.push(">".repeat((target.1 - position.1) as usize));
        }
        if target.0 > position.0 {
            items.push("v".repeat((target.0 - position.0) as usize));
        }
        if target.1 < position.1 {
            items.push("<".repeat((position.1 - target.1) as usize));
        }

        let k = items.len();
        let paths = items.into_iter().permutations(k);
        let mut valid_paths = Vec::new();

        // println!("candidate Paths: {:?}", paths);
        'p_loop: for path in paths {
            let mut c_pos = from;
            for moves in &path {
                for mv in moves.chars() {
                    match mv {
                        'v' => {
                            c_pos.0 += 1;
                            let target = Self::at(keypad, c_pos.0, c_pos.1);
                            if target.is_none() || target.unwrap() == ' ' {
                                continue 'p_loop;
                            }
                        }
                        '^' => {
                            c_pos.0 -= 1;
                            let target = Self::at(keypad, c_pos.0, c_pos.1);
                            if target.is_none() || target.unwrap() == ' ' {
                                continue 'p_loop;
                            }
                        }
                        '<' => {
                            c_pos.1 -= 1;
                            let target = Self::at(keypad, c_pos.0, c_pos.1);
                            if target.is_none() || target.unwrap() == ' ' {
                                continue 'p_loop;
                            }
                        }
                        '>' => {
                            c_pos.1 += 1;
                            let target = Self::at(keypad, c_pos.0, c_pos.1);
                            if target.is_none() || target.unwrap() == ' ' {
                                continue 'p_loop;
                            }
                        }
                        _ => panic!("Unexpected char"),
                    }
                }
            }
            let mut char_path = Vec::new();
            path.iter()
                .for_each(|s| char_path.append(&mut s.chars().collect::<Vec<char>>()));
            valid_paths.push(char_path);
        }
        //valid_paths.sort();
        //valid_paths.dedup();
        valid_paths
    }
}
