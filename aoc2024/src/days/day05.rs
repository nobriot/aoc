use std::ops::Index;

pub fn solve(input: &str) {
    let part_1_total = solve_part_1(input);
    println!("Part 1 Result: {part_1_total}");

    let part_2_total: usize = solve_part_2(input);
    println!("Part 2 Result: {part_2_total}");
}

fn solve_part_1(input: &str) -> usize {
    let mut rules: Vec<Rule> = Vec::new();
    let mut updates: Vec<Update> = Vec::new();
    for line in input.lines() {
        if line.contains("|") {
            rules.push(Rule::from_str(line));
        } else if line.contains(",") {
            updates.push(Update::from_str(line));
        }
    }

    // println!("{} rules, {} updates", rules.len(), updates.len());
    let mut count = 0;
    'updates: for u in updates {
        // println!("u : {:?}, len {}", u, u.len());
        for i in 0..u.len() - 1 {
            // println!("i : {:?}", i);
            for r in rules.iter().as_ref() {
                // println!("Checking rule : {:?}", r);
                if !r.check_rule(&u[0..i], u[i], &u[i + 1..]) {
                    continue 'updates;
                }
            }
        }
        count += u.center_page().unwrap();
    }
    count
}

fn solve_part_2(input: &str) -> usize {
    let mut rules: Vec<Rule> = Vec::new();
    let mut updates: Vec<Update> = Vec::new();
    for line in input.lines() {
        if line.contains("|") {
            rules.push(Rule::from_str(line));
        } else if line.contains(",") {
            updates.push(Update::from_str(line));
        }
    }

    // println!("{} rules, {} updates", rules.len(), updates.len());
    let mut incorrect: Vec<Update> = Vec::new();
    'updates: for u in updates {
        // println!("u : {:?}, len {}", u, u.len());
        for i in 0..u.len() - 1 {
            // println!("i : {:?}", i);
            for r in rules.iter().as_ref() {
                // println!("Checking rule : {:?}", r);
                if !r.check_rule(&u[0..i], u[i], &u[i + 1..]) {
                    incorrect.push(u);
                    continue 'updates;
                }
            }
        }
    }

    // Now fix the incorrect
    let mut count = 0;
    for mut u in incorrect {
        // println!("i : {:?}", i);
        let mut correct = false;
        'correct: while !correct {
            for i in 0..u.len() - 1 {
                for r in rules.iter().as_ref() {
                    // println!("Checking rule : {:?}", r);
                    if !r.check_rule(&u[0..i], u[i], &u[i + 1..]) {
                        u.fix(r);
                    }
                }
                for r in rules.iter().as_ref() {
                    // println!("Checking rule : {:?}", r);
                    if !r.check_rule(&u[0..i], u[i], &u[i + 1..]) {
                        continue 'correct;
                    }
                }
            }
            correct = true;
            count += u.center_page().unwrap();
        }
    }

    count
}

#[derive(Debug)]
struct Rule {
    first: usize,
    second: usize,
}

impl Rule {
    pub fn from_str(input: &str) -> Self {
        let parts = input.split_once("|").expect("Could not split Rule input");

        Rule {
            first: parts.0.parse().expect("Not a number"),
            second: parts.1.parse().expect("Not a number"),
        }
    }

    pub fn contains(&self, a: usize) -> bool {
        self.first == a || self.second == a
    }

    pub fn check_rule(&self, before: &[usize], current: usize, after: &[usize]) -> bool {
        if before.contains(&self.second) && (current == self.first || after.contains(&self.first)) {
            return false;
        }

        if current == self.second && after.contains(&self.first) {
            return false;
        }

        true
    }
}

#[derive(Debug, Clone)]
struct Update {
    numbers: Vec<usize>,
}

impl Update {
    pub fn from_str(input: &str) -> Self {
        let parts = input.split(",");

        let mut numbers: Vec<usize> = Vec::new();
        for part in parts {
            // println!("Adding {}", part);
            numbers.push(part.parse().expect("Could not parse update number"));
        }

        Update { numbers }
    }

    pub fn center_page(&self) -> Option<usize> {
        if self.numbers.is_empty() {
            return None;
        }
        return Some(*self.numbers.get(self.numbers.len() / 2).unwrap());
    }

    pub fn len(&self) -> usize {
        self.numbers.len()
    }

    // Fix an update following an offending rule
    pub fn fix(&mut self, rule: &Rule) {
        assert!(self.numbers.contains(&rule.first));
        assert!(self.numbers.contains(&rule.second));
        // println!("Fixing: {:?} - {}|{}", self, rule.first, rule.second);
        let len = self.len();
        // Pop fron the end into a transfer vector
        let mut transfer: Vec<usize> = Vec::new();
        loop {
            let n = self.numbers.pop();
            assert!(n.is_some());
            let n = n.unwrap();
            if n == rule.second {
                break;
            }
            transfer.push(n);
        }

        // Now start transfering back until we find the first
        loop {
            let n = transfer.pop();
            if n.is_none() {
                break;
            }
            let n = n.unwrap();
            self.numbers.push(n);
            if n == rule.first {
                self.numbers.push(rule.second)
            }
        }
        assert!(self.len() == len, "{:?}", self);
    }
}

impl<I> Index<I> for Update
where
    I: std::slice::SliceIndex<[usize]>,
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.numbers[index]
    }
}
