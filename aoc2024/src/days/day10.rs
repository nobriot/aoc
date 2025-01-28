use crate::input;
use std::collections::HashSet;

pub fn solve() -> (Option<usize>, Option<usize>) {
    let input = input::DAY_10_INPUT;

    let grid = Grid::from_str(input);

    let part_1_total = solve_part_1(&grid);
    let part_2_total = solve_part_2(&grid);

    (part_1_total, part_2_total)
}

fn solve_part_1(grid: &Grid) -> Option<usize> {
    let start_positions = grid.get_number_positions(0);
    let mut count = 0;
    for (l, p) in start_positions {
        count += grid
            .find_trails_ends_from_position(l as isize, p as isize)
            .len();
    }
    Some(count)
}

fn solve_part_2(grid: &Grid) -> Option<usize> {
    let start_positions = grid.get_number_positions(0);
    let mut count = 0;
    for (l, p) in start_positions {
        count += grid.find_trails_from_position(l as isize, p as isize);
    }
    Some(count)
}

#[derive(Debug)]
struct Grid {
    data: Vec<Vec<u32>>,
}

impl Grid {
    fn from_str(input: &str) -> Self {
        let data: Vec<Vec<u32>> = input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();
        Self { data }
    }

    fn at(&self, line: isize, pos: isize) -> Option<u32> {
        if line < 0 || pos < 0 {
            return None;
        }
        let line = line as usize;
        let pos = pos as usize;
        self.data.get(line)?.get(pos).copied()
    }

    // Not sure if I prefer actual for loops or this
    fn get_number_positions(&self, v: u32) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();

        self.data.iter().by_ref().enumerate().for_each(|(l, line)| {
            line.iter()
                .by_ref()
                .enumerate()
                .map(|(p, value)| {
                    if *value == v {
                        return Some((l, p));
                    }
                    None
                })
                .filter(|e| e.is_some())
                .for_each(|e| positions.push(e.unwrap()));
        });
        positions
    }

    fn find_trails_ends_from_position(&self, l: isize, p: isize) -> HashSet<(isize, isize)> {
        let current_gradient = self.at(l, p);
        if current_gradient.is_none() {
            return HashSet::new();
        }
        let current_gradient = current_gradient.unwrap();
        let mut positions = HashSet::new();
        if current_gradient == 9 {
            positions.insert((l, p));
            return positions;
        }

        const DIRECTIONS: [(isize, isize); 4] = [
            (0, 1),  // Right
            (1, 0),  // Down
            (0, -1), // Left
            (-1, 0), // Up
        ];
        for (dl, dp) in &DIRECTIONS {
            if let Some(next_gradient) = self.at(l + dl, p + dp) {
                if next_gradient == current_gradient + 1 {
                    positions.extend(self.find_trails_ends_from_position(l + dl, p + dp));
                }
            }
        }

        positions
    }

    fn find_trails_from_position(&self, l: isize, p: isize) -> usize {
        let current_gradient = self.at(l, p);
        if current_gradient.is_none() {
            return 0;
        }
        let current_gradient = current_gradient.unwrap();
        if current_gradient == 9 {
            return 1;
        }

        let mut count = 0;
        const DIRECTIONS: [(isize, isize); 4] = [
            (0, 1),  // Right
            (1, 0),  // Down
            (0, -1), // Left
            (-1, 0), // Up
        ];
        for (dl, dp) in &DIRECTIONS {
            if let Some(next_gradient) = self.at(l + dl, p + dp) {
                if next_gradient == current_gradient + 1 {
                    count += self.find_trails_from_position(l + dl, p + dp);
                }
            }
        }

        count
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut representation = String::new();
        for line in self.data.iter() {
            for value in line.iter() {
                representation.push(char::from_digit(*value, 10).unwrap());
            }
            representation.push('\n');
        }

        write!(f, "{}", representation)?;
        Ok(())
    }
}
