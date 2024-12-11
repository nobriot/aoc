use crate::input;

pub fn solve() -> (Option<usize>, Option<usize>) {
    let input = input::DAY_7_INPUT;
    let part_1_total = solve_part_1(input);
    //println!("Part 1 Result: {part_1_total}");

    let part_2_total: usize = solve_part_2(input);
    //println!("Part 2 Result: {part_2_total}");

    (Some(part_1_total), Some(part_2_total))
}

fn solve_part_1(input: &str) -> usize {
    let equations = parse_equations_from_str(input);
    count_equations(&equations)
}

fn solve_part_2(input: &str) -> usize {
    let equations = parse_equations_from_str(input);
    count_equations_all_variants(&equations)
}

fn parse_equations_from_str(input: &str) -> Vec<Equation> {
    let mut equations: Vec<Equation> = Vec::new();
    for line in input.lines() {
        // println!("Parsing line: {}", line);
        let parts = line.split_once(":");
        if parts.is_none() {
            continue;
        }

        let parts = parts.unwrap();
        let result: usize = parts.0.parse().unwrap();
        let mut operands: Vec<usize> = Vec::new();

        let parts = parts.1.split(" ");
        for part in parts {
            // println!("Current part {part}");
            if part.trim().is_empty() {
                continue;
            }
            operands.push(part.parse().expect("Could not parse to usize"));
        }

        equations.push(Equation { result, operands });
    }
    equations
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Plus,
    Multiply,
    Concatenate,
}

impl Operator {
    pub fn plus_multiply_variants() -> &'static [Self] {
        &[Self::Plus, Self::Multiply]
    }

    pub fn all_variants() -> &'static [Self] {
        &[Self::Plus, Self::Multiply, Self::Concatenate]
    }

    pub fn generate_combinations(n: usize, variants: &'static [Self]) -> Vec<Vec<Self>> {
        let mut results = Vec::new();
        let mut current: Vec<Operator> = Vec::new();

        Self::iterate(&mut results, &mut current, n, variants);
        results
    }

    fn iterate(
        results: &mut Vec<Vec<Operator>>,
        current: &mut Vec<Operator>,
        remaining: usize,
        variants: &'static [Self],
    ) {
        if remaining == 0 {
            results.push(current.clone());
            return;
        }

        for &variant in variants {
            current.push(variant);
            Self::iterate(results, current, remaining - 1, variants);
            current.pop();
        }
    }
}

#[derive(Debug)]
struct Equation {
    result: usize,
    operands: Vec<usize>,
}

impl Equation {
    // Determines if operands can be combined to give the result
    pub fn is_possible(&self) -> bool {
        // println!("Length : {}", self.operands.len() - 1);

        for permutation in Operator::generate_combinations(
            self.operands.len() - 1,
            Operator::plus_multiply_variants(),
        ) {
            if self.calculate_if_equal(&permutation) {
                return true;
            }
        }
        false
    }

    pub fn is_possible_extended(&self) -> bool {
        for permutation in
            Operator::generate_combinations(self.operands.len() - 1, Operator::all_variants())
        {
            // println!("Trying {:?}", permutation);
            if self.calculate_if_equal(&permutation) {
                return true;
            }
        }
        false
    }
    /// Calculate the if the results matches given a slice of operators
    /// early abort if the current calculation is too large.
    fn calculate_if_equal(&self, operators: &[Operator]) -> bool {
        debug_assert!(operators.len() == self.operands.len() - 1);

        let mut total = self.operands[0];
        for (i, operator) in operators.iter().enumerate() {
            match operator {
                Operator::Plus => total += self.operands[i + 1],
                Operator::Multiply => total *= self.operands[i + 1],
                Operator::Concatenate => {
                    total =
                        total * 10_usize.pow(digits(self.operands[i + 1])) + self.operands[i + 1]
                }
            }

            if total > self.result {
                return false;
            }
        }

        if total == self.result {
            /*println!(
                "Matching: {:?} = {:?} with {:?}",
                self.result, self.operands, operators
            ); */
            return true;
        }
        false
    }
}

fn count_equations(equations: &Vec<Equation>) -> usize {
    let mut count: usize = 0;
    for equation in equations {
        // println!("Checking {:?}", equation);
        if equation.is_possible() {
            count += equation.result;
        }
    }
    count
}

fn count_equations_all_variants(equations: &Vec<Equation>) -> usize {
    let mut count: usize = 0;
    for equation in equations {
        // println!("Checking {:?}", equation);
        if equation.is_possible_extended() {
            count += equation.result;
        }
    }
    count
}

// counts the numbers of digits in base 10 for a usize
fn digits(a: usize) -> u32 {
    let mut remainder = a;
    let mut digits = 1;
    while remainder >= 10 {
        remainder /= 10;
        digits += 1;
    }
    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combinations() {
        let p = Operator::generate_combinations(3, Operator::all_variants());

        assert!(p.len() == 3_usize.pow(3));

        let p = Operator::generate_combinations(7, Operator::all_variants());
        assert!(p.len() == 3_usize.pow(7));

        let p = Operator::generate_combinations(7, Operator::plus_multiply_variants());
        assert!(p.len() == 2_usize.pow(7));
    }
}
