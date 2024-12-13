use crate::input;
use regex::Regex;

const BUTTON_A_REGEX: &str = r#"Button A: X\+(?<left>\d+), Y\+(?<right>\d+)"#;
const BUTTON_B_REGEX: &str = r#"Button B: X\+(?<left>\d+), Y\+(?<right>\d+)"#;
const PRIZE_REGEX: &str = r#"Prize: X=(?<left>\d+), Y=(?<right>\d+)"#;

pub fn solve() -> (Option<usize>, Option<usize>) {
    let input = input::DAY_13_INPUT;
    let equations = Equations::from_str(input);
    let part_1_total = solve_part_1(&equations);
    let part_2_total = solve_part_2(&equations);
    (part_1_total, part_2_total)
}

fn solve_part_1(equations: &Equations) -> Option<usize> {
    Some(equations.calculate_price())
}

fn solve_part_2(equations: &Equations) -> Option<usize> {
    Some(equations.calculate_price_calibration())
}

#[derive(Debug, Clone)]
struct Equation {
    /// A (X1, Y1)
    a: (isize, isize),
    /// B (X2, Y2)
    b: (isize, isize),
    /// Prize (Px, Py)
    prize: (isize, isize),
}

impl Equation {
    const A_TOKENS: usize = 3;
    const B_TOKENS: usize = 1;
    const CALIBRATION: isize = 10000000000000;

    /// Solve the equation for the given values of a and b
    /// We have
    /// (1) Px = A.X1 + B.X2
    /// (2) Py = A.Y1 + B.Y2
    ///
    /// (1) => Px = A.X1 + B.X2
    ///     => Px - B.X2 = A.X1
    ///     => (Px - B.X2) / X1 = A
    /// Inject into (2)
    ///     => Py = ((Px - B.X2) / X1) .Y1 + B.Y2
    ///     => Py = ((Px.Y1 - B.X2.Y1) / X1) + B.Y2
    ///     => X1.Py = (Px.Y1 - B.X2.Y1)  + B.Y2.X1
    ///     => X1.Py = Px.Y1 + B ( -X2.Y1  + Y2.X1)
    ///     => X1.Py - Px.Y1 =  B ( -X2.Y1  + Y2.X1)
    ///     => (X1.Py - Px.Y1) / ( -X2.Y1  + Y2.X1) = B
    ///
    #[allow(non_snake_case)]
    pub fn solve_less_than_100(&self) -> Option<(usize, usize)> {
        let B = (self.a.0 * self.prize.1 - self.prize.0 * self.a.1)
            / (self.b.1 * self.a.0 - self.a.1 * self.b.0);
        let A = (self.prize.0 - self.b.0 * B) / self.a.0;

        // Number of presses has to be positive
        if A < 0 || B < 0 {
            // println!("Unexpected: A: {}, B: {}", A, B);
            return None;
        }
        if A > 100 || B > 100 {
            // println!("Too high: A: {}, B: {}", A, B);
            return None;
        }

        // divisions can also give some possible solutions due to rounding errors with isize types.
        // Discard solutions if they do not add up to the result
        if !(self.prize.0 == A * self.a.0 + B * self.b.0) {
            return None;
        }
        if !(self.prize.1 == A * self.a.1 + B * self.b.1) {
            return None;
        }

        Some((A as usize, B as usize))
    }

    /// Kind of the same as solve_less_than_100, just modifying contraints and prize value
    #[allow(non_snake_case)]
    pub fn solve_part_2(&self) -> Option<(usize, usize)> {
        let prize = (
            self.prize.0 + Self::CALIBRATION,
            self.prize.1 + Self::CALIBRATION,
        );

        let B =
            (self.a.0 * prize.1 - prize.0 * self.a.1) / (self.b.1 * self.a.0 - self.a.1 * self.b.0);
        let A = (prize.0 - self.b.0 * B) / self.a.0;

        // Number of presses has to be positive
        if A < 0 || B < 0 {
            // println!("Unexpected: A: {}, B: {}", A, B);
            return None;
        }

        // divisions can also give some possible solutions due to rounding errors with isize types.
        // Discard solutions if they do not add up to the result
        if !(prize.0 == A * self.a.0 + B * self.b.0) {
            return None;
        }
        if !(prize.1 == A * self.a.1 + B * self.b.1) {
            return None;
        }

        Some((A as usize, B as usize))
    }
}

#[derive(Debug, Clone)]
struct Equations {
    equations: Vec<Equation>,
}

impl Equations {
    pub fn from_str(input: &str) -> Self {
        let button_a_re = Regex::new(BUTTON_A_REGEX).unwrap();
        let button_b_re = Regex::new(BUTTON_B_REGEX).unwrap();
        let prize_re = Regex::new(PRIZE_REGEX).unwrap();

        let mut equations: Vec<Equation> = Vec::new();
        let mut a: Option<(isize, isize)> = None;
        let mut b: Option<(isize, isize)> = None;

        // Ugly parsing, there is probably much better
        for line in input.lines() {
            // println!("Line {}: {}", i, line);
            if line.is_empty() {
                continue;
            }

            if line.contains("Button A") {
                let caps = button_a_re.captures(line).unwrap();
                let x: isize = caps["right"].parse().unwrap();
                let y: isize = caps["left"].parse().unwrap();
                a = Some((x, y));
            } else if line.contains("Button B") {
                let caps = button_b_re.captures(line).unwrap();
                let x: isize = caps["right"].parse().unwrap();
                let y: isize = caps["left"].parse().unwrap();
                b = Some((x, y))
            } else if line.contains("Prize") {
                let caps = prize_re.captures(line).unwrap();
                let x: isize = caps["right"].parse().unwrap();
                let y: isize = caps["left"].parse().unwrap();
                equations.push(Equation {
                    a: a.unwrap(),
                    b: b.unwrap(),
                    prize: (x, y),
                });
                a = None;
                b = None;
            }
        }
        Self { equations }
    }

    pub fn calculate_price(&self) -> usize {
        let mut count = 0;
        self.equations.iter().for_each(|eq| {
            // println!("Solving equation: {:?}", eq);
            let solution = eq.solve_less_than_100();
            // println!("Solution: {:?}", solution);
            if solution.is_none() {
                return;
            }
            let solution = solution.unwrap();
            count += Equation::A_TOKENS * solution.0 + Equation::B_TOKENS * solution.1;
            // println!("Count: {}", count);
        });
        count
    }

    pub fn calculate_price_calibration(&self) -> usize {
        let mut count = 0;
        self.equations.iter().for_each(|eq| {
            // println!("Solving equation: {:?}", eq);
            let solution = eq.solve_part_2();
            //println!("Solution: {:?}", solution);
            if solution.is_none() {
                return;
            }
            let solution = solution.unwrap();
            count += Equation::A_TOKENS * solution.0 + Equation::B_TOKENS * solution.1;
        });
        count
    }
}
