pub fn solve(input: &str) {
    let part_1_total = solve_part_1(input);
    println!("Part 1 Result: {part_1_total}");

    let part_2_total: usize = solve_part_2(input);
    println!("Part 2 Result: {part_2_total}");
}

pub fn solve_part_1(input: &str) -> usize {
    let grid = Grid::from_str(input);
    grid.count_word(&['X', 'M', 'A', 'S'])
}

pub fn solve_part_2(input: &str) -> usize {
    let grid = Grid::from_str(input);
    grid.find_crosses('A', &['M', 'A', 'S'])
}

struct Grid {
    data: Vec<Vec<char>>,
}

impl Grid {
    fn from_str(input: &str) -> Self {
        let data = input.lines().map(|l| l.chars().collect()).collect();
        Self { data }
    }

    fn at(&self, line: usize, pos: usize) -> Option<char> {
        self.data.get(line)?.get(pos).copied()
    }

    fn count_word(&self, word: &[char]) -> usize {
        let mut count: usize = 0;
        for (l, line) in self.data.clone().into_iter().enumerate() {
            for (p, ch) in line.into_iter().enumerate() {
                if ch == word[0] {
                    count += self.count_word_directions(l, p, word);
                }
            }
        }
        count
    }

    fn count_word_directions(&self, line: usize, pos: usize, word: &[char]) -> usize {
        debug_assert!(self.at(line, pos).unwrap() == word[0]);
        let mut count: usize = 0;

        // Could probably be better ...
        let directions = [
            (0, 1),   // Right
            (1, 1),   // Down-right
            (1, 0),   // Down
            (1, -1),  // Down-left
            (0, -1),  // Left
            (-1, -1), // Up-left
            (-1, 0),  // Up
            (-1, 1),  // Up-right
        ];
        for &(line_delta, pos_delta) in &directions {
            if self.check_word_direction(line, line_delta, pos, pos_delta, word) {
                count += 1;
            }
        }

        count
    }

    fn check_word_direction(
        &self,
        line: usize,
        line_delta: isize,
        pos: usize,
        pos_delta: isize,
        word: &[char],
    ) -> bool {
        for (i, expected_char) in word.iter().enumerate() {
            let l: isize = line as isize + line_delta * i as isize;
            let p: isize = pos as isize + pos_delta * i as isize;

            if p < 0 || l < 0 {
                return false;
            }

            let actual = self.at(l as usize, p as usize);
            if actual.is_none() {
                return false;
            }
            if actual.unwrap() != *expected_char {
                return false;
            }
        }

        true
    }

    fn find_crosses(&self, center: char, word: &[char]) -> usize {
        let mut count: usize = 0;
        for (l, line) in self.data.clone().into_iter().enumerate() {
            for (p, ch) in line.into_iter().enumerate() {
                if ch == center && self.check_diagonals(l, p, word) {
                    count += 1;
                }
            }
        }
        count
    }

    fn check_diagonals(&self, line: usize, pos: usize, word: &[char]) -> bool {
        let mut word_reverse: Vec<char> = Vec::with_capacity(word.len());
        for c in word.iter().rev() {
            word_reverse.push(*c);
        }

        // Diagonal 1:
        let l1 = line as isize - (word.len() as isize) / 2;
        let p1 = pos as isize - (word.len() as isize) / 2;

        if p1 < 0 || l1 < 0 {
            return false;
        }

        // Diagonal 2:
        let l2 = line as isize + (word.len() as isize) / 2;
        let p2 = pos as isize - (word.len() as isize) / 2;

        if p2 < 0 || l2 < 0 {
            return false;
        }

        // I will have no idea what is going on if I look at this in 6 month from now :grin:
        if !self.check_word_direction(l1 as usize, 1, p1 as usize, 1, word)
            && !self.check_word_direction(l1 as usize, 1, p1 as usize, 1, &word_reverse)
        {
            return false;
        }
        if !self.check_word_direction(l2 as usize, -1, p2 as usize, 1, word)
            && !self.check_word_direction(l2 as usize, -1, p2 as usize, 1, &word_reverse)
        {
            return false;
        }
        true
    }
}
