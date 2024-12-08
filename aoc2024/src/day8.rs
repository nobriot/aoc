pub fn solve(input: &str) {
    let grid = Grid::from_str(input);
    let part_1_total = solve_part_1(grid);
    println!("Part 1 Result: {part_1_total}");

    let part_2_total: usize = solve_part_2(&input);
    println!("Part 2 Result: {part_2_total}");
}

fn solve_part_1(grid: Grid) -> usize {
    grid.calculate_antinodes()
}

fn solve_part_2(input: &str) -> usize {
    todo!();
}

#[derive(Debug)]
struct Grid {
    data: Vec<Vec<char>>,
}

impl Grid {
    fn from_str(input: &str) -> Self {
        let data = input.lines().map(|l| l.chars().collect()).collect();
        Self { data }
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
    fn calculate_antinodes(mut self) -> usize {
        let mut count = 0;
        let mut completed = false;

        while !completed {
            println!("Completed loop");
            let mut current = (0, 0);

            'lines: for (l, line) in self.data.iter().enumerate() {
                println!("l: {}, current: {}", l, current.0);
                if l < current.0 {
                    continue;
                }
                for (p, ch) in line.iter().enumerate() {
                    if l < current.1 {
                        continue;
                    }
                    if *ch == '.' {
                        continue;
                    }

                    // Find all identical nodes
                    let others = self.get_char_positions(*ch);

                    for other in others {
                        let positions = Grid::get_antinode_positions((l, p), other);

                        for position in positions {
                            if self.at(position.0, position.1).is_some() {
                                count += 1;
                            }
                        }
                    }

                    // Remove the node:
                    current = (l, p);
                    continue 'lines;
                }
            }
            completed = true;
        }
        count
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

        if p1 != a_i && p1 != b_i && p1.0 >= 0 && p1.1 >= 0 {
            positions.push(p1);
        }

        if p2 != a_i && p2 != b_i && p2.0 >= 0 && p2.1 >= 0 {
            positions.push(p2);
        }

        if p3 != a_i && p3 != b_i && p3.0 >= 0 && p3.1 >= 0 {
            positions.push(p3);
        }

        if p4 != a_i && p4 != b_i && p4.0 >= 0 && p4.1 >= 0 {
            positions.push(p4);
        }
        positions
    }

    fn get_char_positions(&self, c: char) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();
        for (l, line) in self.data.iter().by_ref().enumerate() {
            for (p, ch) in line.iter().by_ref().enumerate() {
                if *ch == c {
                    positions.push((l, p));
                }
            }
        }
        positions
    }
}
