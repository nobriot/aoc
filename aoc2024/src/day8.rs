use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) {
    let grid = Grid::from_str(input);
    let part_1_total = solve_part_1(grid);
    println!("Part 1 Result: {part_1_total}");

    let grid = Grid::from_str(input);
    let part_2_total = solve_part_2(grid);
    println!("Part 2 Result: {part_2_total}");
}

fn solve_part_1(grid: Grid) -> usize {
    grid.calculate_antinodes()
}

fn solve_part_2(grid: Grid) -> usize {
    grid.calculate_extended_antinodes()
}

#[derive(Debug)]
struct Grid {
    data: Vec<Vec<char>>,
    lines: usize,
    cols: usize,
}

impl Grid {
    fn from_str(input: &str) -> Self {
        let data = input.lines().map(|l| l.chars().collect()).collect();
        let lines = data.len();
        let cols = data[0].len();
        Self { data, lines, cols }
    }

    fn at(&self, line: isize, pos: isize) -> Option<char> {
        if line < 0 || pos < 0 {
            return None;
        }
        let line = line as usize;
        let pos = pos as usize;
        self.data.get(line)?.get(pos).copied()
    }

    fn set(&mut self, line: usize, pos: usize, c: char) {
        if self.at(line as isize, pos as isize).is_some() {
            self.data.get_mut(line).unwrap()[pos] = c;
        } else {
            eprintln!("Got out of bound set {} {}", line, pos);
        }
    }

    /// Consumes the object at it erase all nodes
    fn calculate_antinodes(self) -> usize {
        let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
        let mut current = (0, 0);
        for (l, line) in self.data.iter().enumerate() {
            if l < current.0 {
                continue;
            }
            for (p, ch) in line.iter().enumerate() {
                if l == current.0 && p < current.1 {
                    continue;
                }
                if *ch == '.' {
                    continue;
                }
                println!("l: {}, p : {}, current: {:?} - char {}", l, p, current, *ch);

                // Find all identical nodes
                let others = self.get_char_positions(*ch, l, p + 1);

                println!("counterparts: {}", others.len());
                for other in others {
                    let positions = Grid::get_antinode_positions((l, p), other);
                    assert!(positions.len() == 2);
                    println!("{}, Antinode positions: {:?}", antinodes.len(), positions);
                    for position in positions {
                        if self.at(position.0, position.1).is_some() {
                            antinodes.insert((position.0, position.1));
                        }
                    }
                    println!("Updated total:{}", antinodes.len());
                }

                // Mark progress
                current = (l, p);
            }
        }
        antinodes.len()
    }

    /// Consumes the object at it erase all nodes
    fn calculate_extended_antinodes(self) -> usize {
        let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
        let mut current = (0, 0);
        for (l, line) in self.data.iter().enumerate() {
            if l < current.0 {
                continue;
            }
            for (p, ch) in line.iter().enumerate() {
                if l == current.0 && p < current.1 {
                    continue;
                }
                if *ch == '.' {
                    continue;
                }
                println!("l: {}, p : {}, current: {:?} - char {}", l, p, current, *ch);

                // Find all identical nodes
                let others = self.get_char_positions(*ch, l, p + 1);

                println!("counterparts: {}", others.len());
                for other in others {
                    let positions = Grid::get_extended_antinode_positions((l, p), other);
                    println!("{}, Antinode positions: {:?}", antinodes.len(), positions);
                    for position in positions {
                        if self.at(position.0, position.1).is_some() {
                            antinodes.insert((position.0, position.1));
                        }
                    }
                    println!("Updated total:{}", antinodes.len());
                }

                // Mark progress
                current = (l, p);
            }
        }
        antinodes.len()
    }
    fn get_antinode_positions(a: (usize, usize), b: (usize, usize)) -> Vec<(isize, isize)> {
        let mut positions = Vec::new();

        let a_i = (a.0 as isize, a.1 as isize);
        let b_i = (b.0 as isize, b.1 as isize);

        let delta = (a_i.0 - b_i.0, a_i.1 - b_i.1);

        let p1 = (a_i.0 - delta.0, a_i.1 - delta.1);
        let p2 = (a_i.0 + delta.0, a_i.1 + delta.1);
        let p3 = (b_i.0 - delta.0, b_i.1 - delta.1);
        let p4 = (b_i.0 + delta.0, b_i.1 + delta.1);

        if p1 != a_i && p1 != b_i {
            positions.push(p1);
        }

        if p2 != a_i && p2 != b_i {
            positions.push(p2);
        }

        if p3 != a_i && p3 != b_i {
            positions.push(p3);
        }

        if p4 != a_i && p4 != b_i {
            positions.push(p4);
        }
        positions
    }

    fn get_extended_antinode_positions(
        a: (usize, usize),
        b: (usize, usize),
    ) -> Vec<(isize, isize)> {
        let mut positions = Vec::new();

        let a_i = (a.0 as isize, a.1 as isize);
        let b_i = (b.0 as isize, b.1 as isize);

        let delta = (a_i.0 - b_i.0, a_i.1 - b_i.1);

        let p1 = (a_i.0 - delta.0, a_i.1 - delta.1);
        let p2 = (a_i.0 + delta.0, a_i.1 + delta.1);
        let p3 = (b_i.0 - delta.0, b_i.1 - delta.1);
        let p4 = (b_i.0 + delta.0, b_i.1 + delta.1);

        if p1 != a_i && p1 != b_i {
            positions.push(p1);
        }

        if p2 != a_i && p2 != b_i {
            positions.push(p2);
        }

        if p3 != a_i && p3 != b_i {
            positions.push(p3);
        }

        if p4 != a_i && p4 != b_i {
            positions.push(p4);
        }
        positions
    }
    fn get_char_positions(&self, c: char, l_min: usize, p_min: usize) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();
        for (l, line) in self.data.iter().by_ref().enumerate() {
            if l <= l_min {
                continue;
            }
            for (p, ch) in line.iter().by_ref().enumerate() {
                if l == l_min && p < p_min {
                    continue;
                }
                if *ch == c {
                    positions.push((l, p));
                }
            }
        }
        positions
    }
}
