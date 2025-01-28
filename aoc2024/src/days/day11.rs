use crate::input;
use std::collections::HashMap;

pub fn solve() -> (Option<usize>, Option<usize>) {
    let input = input::DAY_11_INPUT;

    let part_1_total = solve_part_1(&input);
    let part_2_total = solve_part_2(&input);

    (part_1_total, part_2_total)
}

fn solve_part_1(input: &str) -> Option<usize> {
    let mut stones = Stones::from_str(input);
    for _ in 0..25 {
        stones.blink();
    }
    Some(stones.len())
}

fn solve_part_2(input: &str) -> Option<usize> {
    let mut stones = Stones::from_str(input);
    for _ in 0..75 {
        stones.blink();
    }
    Some(stones.len())
}

#[derive(Debug, Clone)]
struct Stones {
    stones: HashMap<usize, isize>,
}

impl Stones {
    pub fn new() -> Self {
        Self {
            stones: HashMap::new(),
        }
    }
    pub fn from_str(input: &str) -> Self {
        let mut stones = Self {
            stones: HashMap::new(),
        };

        for d in input.split(" ").filter_map(|s| s.trim().parse().ok()) {
            stones.update(d, 1);
        }
        stones
    }

    pub fn update(&mut self, key: usize, value: isize) {
        let previous_value = self.stones.get(&key);
        match previous_value {
            Some(i) => {
                if (*i + value) < 1 {
                    let _ = self.stones.remove(&key);
                } else {
                    let _ = self.stones.insert(key, i + value);
                }
            }
            None => {
                let _ = self.stones.insert(key, value);
            }
        }
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        for (_, value) in &self.stones {
            count += value;
        }
        count as usize
    }

    pub fn blink(&mut self) {
        let mut new_stones = self.clone();

        for (stone, count) in &self.stones {
            // Decrement ongoing stone
            new_stones.update(*stone, -*count);

            // Rule 1
            if *stone == 0 {
                new_stones.update(1, *count);
                continue;
            }

            // Rule 2
            if let Some((part1, part2)) = Stones::split_digits(*stone) {
                new_stones.update(part1, *count);
                new_stones.update(part2, *count);
                continue;
            }

            // Rule 3
            new_stones.update(*stone * 2024, *count);
        }

        *self = new_stones
    }

    pub fn split_digits(stone: usize) -> Option<(usize, usize)> {
        let mut remainder = stone;
        let mut digits = 1;
        while remainder >= 10 {
            remainder /= 10;
            digits += 1;
        }

        if digits % 2 == 1 {
            return None;
        }

        let mut divider = 1;
        for _ in 0..digits / 2 {
            divider *= 10;
        }

        let part2 = stone % divider;
        let part1 = stone / divider;

        Some((part1, part2))
    }
}
