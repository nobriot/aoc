use crate::input;
use std::{collections::HashSet, hash::Hash};

pub fn solve() -> (Option<usize>, Option<usize>) {
    let input = input::DAY_19_INPUT;
    let patterns = Patterns::from_str(input.lines().nth(0).expect("File is not empty"));
    let designs = Designs::from_str(input);
    println!(
        "Patterns: {:?}, Designs: {:?}",
        patterns.patterns.len(),
        designs.designs.len()
    );

    let part_1_total = solve_part_1(&patterns, &designs);

    let part_2_total = solve_part_2(&patterns, &designs);

    (part_1_total, part_2_total)
}

fn solve_part_1(patterns: &Patterns, designs: &Designs) -> Option<usize> {
    Some(designs.count_possible_designs(patterns))
}

fn solve_part_2(patterns: &Patterns, designs: &Designs) -> Option<usize> {
    None
}

#[derive(Debug)]
struct Patterns {
    patterns: Vec<String>,
}

impl Patterns {
    pub fn from_str(input: &str) -> Self {
        let mut patterns: Vec<String> = input.split(",").map(|s| s.trim().to_string()).collect();
        patterns.dedup_by(|a, b| a == b);
        patterns.retain(|s| !s.is_empty());
        Self { patterns }
    }
}

#[derive(Debug)]
struct Designs {
    designs: Vec<String>,
}

impl Designs {
    pub fn from_str(input: &str) -> Self {
        let designs = input
            .lines()
            .filter(|l| !l.is_empty() && !l.contains(","))
            .map(|s| s.trim().to_string())
            .collect();
        Self { designs }
    }

    pub fn count_possible_designs(&self, patterns: &Patterns) -> usize {
        let mut count = 0;

        self.designs.iter().for_each(|design| {
            print!("Design: {:?} - ", design);
            let mut visited: HashSet<String> = HashSet::new();
            if Self::is_design_possible(design, None, &mut visited, patterns) {
                println!("possible");
                count += 1
            } else {
                println!("not possible");
            }
            // panic!();
        });

        count
    }

    fn is_design_possible(
        design: &str,
        current: Option<&str>,
        visited: &mut HashSet<String>,
        patterns: &Patterns,
    ) -> bool {
        let mut current = String::from(current.unwrap_or(""));

        if visited.contains(&current) {
            return false;
        }

        // println!("Current: {:?}", current);
        for pattern in &patterns.patterns {
            // println!("pattern: {:?}", pattern);
            current.push_str(pattern);
            //println!("New current: {:?} ", current);

            if current.len() < design.len() && design[..current.len()] == current {
                if Self::is_design_possible(design, Some(&current), visited, patterns) {
                    return true;
                }
            } else if design == current {
                return true;
            }

            // Pop the pattern we just added
            for _ in 0..pattern.len() {
                current.pop();
            }
        }
        visited.insert(current.clone());

        false
    }
}
