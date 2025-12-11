use crate::input;

const MAX_ANGLE: isize = 100;

#[derive(Debug, Copy, Clone, Default)]
pub struct Dial {
    pub angle: isize,
    pub exact_zeroes: usize,
    pub any_zeroes: usize,
}

impl Dial {
    pub fn new() -> Self {
        Self {
            angle: 50,
            exact_zeroes: 0,
            any_zeroes: 0,
        }
    }

    pub fn update(&mut self, movement: isize) {
        // This does not happen I believe, but who knows
        if movement == 0 {
            return;
        }

        let was_zero = self.angle == 0;
        self.angle += movement;

        // Post-process the increment and count the clicks
        if self.angle >= MAX_ANGLE {
            let count = self.angle / MAX_ANGLE;
            self.angle -= count * MAX_ANGLE;
            self.any_zeroes += count as usize;
        } else if self.angle < 0 {
            let mut count = -(self.angle / MAX_ANGLE) + 1;
            if -self.angle % MAX_ANGLE == 0 {
                count -= 1;
            }
            self.angle += count * MAX_ANGLE;
            let mut increment = if self.angle == 0 { count + 1 } else { count };
            if was_zero {
                increment -= 1;
            }
            self.any_zeroes += increment as usize;
        } else if self.angle == 0 {
            self.any_zeroes += 1;
        }

        // Some sanity check
        assert!(self.angle >= 0 && self.angle < MAX_ANGLE);

        // Count exact zeroes after movements
        if self.angle == 0 {
            self.exact_zeroes += 1;
        }
    }
}

pub fn solve() -> (Option<usize>, Option<usize>) {
    let input = input::DAY_1_INPUT;

    let mut movements = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            break;
        }

        let sign: isize = match line.chars().next().expect("line is not empty") {
            'L' => -1,
            'R' => 1,
            _ => panic!("Unknown rotation: {line}"),
        };

        // Collect the rest and parse as isize:
        // We know that the first char is L or R, so u8 size
        let increment = line[1..].parse::<isize>().unwrap();
        movements.push(sign * increment);
    }

    let mut dial = Dial::new();
    for mv in movements {
        dial.update(mv);
    }

    (Some(dial.exact_zeroes), Some(dial.any_zeroes))
}
