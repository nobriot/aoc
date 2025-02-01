use crate::input;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve() -> (Option<usize>, Option<usize>) {
    let input = input::DAY_19_INPUT;
    let patterns = Patterns::from_str(input.lines().next().expect("File is not empty"));
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
    Some(designs.count_all_designs(patterns))
}

#[derive(Debug)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
}

impl TrieNode {
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_end: false,
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut current = self;
        for c in word.chars() {
            current = current.children.entry(c).or_insert(TrieNode::new());
        }
        current.is_end = true;
    }
}

#[derive(Debug)]
struct Patterns {
    patterns: Vec<String>,
    trie: TrieNode,
}

impl Patterns {
    pub fn from_str(input: &str) -> Self {
        let mut patterns: Vec<String> = input.split(",").map(|s| s.trim().to_string()).collect();
        patterns.dedup_by(|a, b| a == b);
        patterns.retain(|s| !s.is_empty());

        let mut trie = TrieNode::new();
        for pattern in &patterns {
            trie.insert(pattern);
        }

        Self { patterns, trie }
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
            //print!("Design: {:?} - ", design);
            let mut visited: HashSet<String> = HashSet::new();
            if Self::is_design_possible(design, None, &mut visited, patterns) {
                //println!("possible");
                count += 1
            } else {
                //println!("not possible");
            }
        });

        count
    }

    pub fn count_all_designs(&self, patterns: &Patterns) -> usize {
        let mut count = 0;

        self.designs.iter().for_each(|design| {
            // print!("Design: {:?} - ", design);
            let mut visited: HashMap<String, usize> = HashMap::new();
            count += Self::count_design_combinations(design, design, &mut visited, patterns);
            // println!("Design: {:?} - Count: {:?}", design, count);
            // println!("Visited: {:?} ", visited);
        });

        count
    }

    fn count_design_combinations(
        design: &str,
        remaining: &str,
        visited: &mut HashMap<String, usize>,
        patterns: &Patterns,
    ) -> usize {
        // Current is the start of design minus remaining
        let current = String::from(&design[..design.len() - remaining.len()]);
        // println!("Current: {:?} - Remaining: {:?}", current, remaining);

        if let Some(&cached_result) = visited.get(&current) {
            return cached_result;
        }

        let mut count = 0;
        let mut node = &patterns.trie;

        for (i, c) in remaining.chars().enumerate() {
            match node.children.get(&c) {
                Some(child) => {
                    node = child;

                    if node.is_end {
                        let new_remaining = &remaining[i + 1..];
                        if new_remaining.is_empty() {
                            count += 1;
                        } else {
                            count += Self::count_design_combinations(
                                design,
                                new_remaining,
                                visited,
                                patterns,
                            );
                        }
                    }
                }
                None => {
                    break;
                }
            }
        }

        visited.insert(current, count);
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
        visited.insert(current.clone());

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

        false
    }
}
