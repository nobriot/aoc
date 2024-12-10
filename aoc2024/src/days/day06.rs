use std::collections::HashSet;

pub fn solve(input: &str) {
    let grid = Grid::from_str(input);

    let part_1_total = solve_part_1(&grid);
    println!("Part 1 Result: {part_1_total}");

    let part_2_total: usize = solve_part_2(&grid);
    println!("Part 2 Result: {part_2_total}");
}

fn solve_part_1(grid: &Grid) -> usize {
    let mut guard = Guard::from_grid(grid).expect("Could not find the guard");
    let _ = guard.walk(grid);

    guard.covered_ground()
}

fn solve_part_2(grid: &Grid) -> usize {
    let area = grid.area();
    println!("The grid is {} squares", area);

    let obstacle_positions = grid.get_char_positions('.');
    let g1 = grid.get_char_positions('^');
    let o1 = grid.get_char_positions('#');
    println!(
        "There are {} possible obstacles ({} {})",
        obstacle_positions.len(),
        g1.len(),
        o1.len()
    );

    let mut stuck_count = 0;
    for (l, p) in obstacle_positions {
        let mut new_grid = grid.clone();
        let mut guard = Guard::from_grid(grid).expect("Could not find the guard");
        new_grid.set(l, p, '#');
        if guard.walk(&new_grid).is_none() {
            stuck_count += 1;
        }
    }

    stuck_count
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

    fn at(&self, line: usize, pos: usize) -> Option<char> {
        self.data.get(line)?.get(pos).copied()
    }

    fn set(&mut self, line: usize, pos: usize, c: char) {
        if self.at(line, pos).is_some() {
            self.data.get_mut(line).unwrap()[pos] = c;
        } else {
            eprintln!("Got out of bound set {} {}", line, pos);
        }
    }

    // Ugly way to duplicate grid
    fn clone(&self) -> Self {
        let mut data = Vec::new();

        for line in self.data.iter().by_ref() {
            let mut inner_data = Vec::new();
            for ch in line.iter().by_ref() {
                inner_data.push(*ch);
            }
            data.push(inner_data);
        }

        Self { data }
    }

    fn find_char(&self, c: char) -> Option<(usize, usize)> {
        for (l, line) in self.data.iter().by_ref().enumerate() {
            for (p, ch) in line.iter().by_ref().enumerate() {
                if *ch == c {
                    return Some((l, p));
                }
            }
        }
        None
    }

    fn area(&self) -> usize {
        let mut area = 0;
        for line in self.data.iter().by_ref() {
            for _ in line.iter().by_ref() {
                area += 1;
            }
        }
        area
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&mut self) {
        match self {
            Direction::Up => *self = Direction::Right,
            Direction::Right => *self = Direction::Down,
            Direction::Down => *self = Direction::Left,
            Direction::Left => *self = Direction::Up,
        }
    }
}

#[derive(Debug)]
struct Guard {
    line: usize,
    pos: usize,
    dir: Direction,
    visited: HashSet<(usize, usize)>,
    obstacles: HashSet<(usize, usize, Direction)>,
}

impl Guard {
    fn from_grid(grid: &Grid) -> Option<Self> {
        let opt = grid.find_char('^');
        if opt.is_some() {
            let (line, pos) = opt.unwrap();
            let mut visited = HashSet::new();
            visited.insert((line, pos));
            return Some(Self {
                line,
                pos,
                dir: Direction::Up,
                visited,
                obstacles: HashSet::new(),
            });
        }
        let opt = grid.find_char('>');
        if opt.is_some() {
            let (line, pos) = opt.unwrap();
            let mut visited = HashSet::new();
            visited.insert((line, pos));
            return Some(Self {
                line,
                pos,
                dir: Direction::Right,
                visited,
                obstacles: HashSet::new(),
            });
        }
        let opt = grid.find_char('v');
        if opt.is_some() {
            let (line, pos) = opt.unwrap();
            let mut visited = HashSet::new();
            visited.insert((line, pos));
            return Some(Self {
                line,
                pos,
                dir: Direction::Down,
                visited,
                obstacles: HashSet::new(),
            });
        }
        let opt = grid.find_char('<');
        if opt.is_some() {
            let (line, pos) = opt.unwrap();
            let mut visited = HashSet::new();
            visited.insert((line, pos));
            return Some(Self {
                line,
                pos,
                dir: Direction::Left,
                visited,
                obstacles: HashSet::new(),
            });
        }
        None
    }

    // This count how many steps the guard is doing
    // Returns None if the guard gets stuck in circles
    fn walk(&mut self, grid: &Grid) -> Option<usize> {
        // Find the next position when walking
        let (l, p) = match self.dir {
            Direction::Up => (self.line as isize - 1, self.pos as isize),
            Direction::Right => (self.line as isize, self.pos as isize + 1),
            Direction::Down => (self.line as isize + 1, self.pos as isize),
            Direction::Left => (self.line as isize, self.pos as isize - 1),
        };
        // Check getting out of bound (cast as usize won't work)
        if l < 0 || p < 0 {
            return Some(0);
        }

        // Look what lays ahead
        let l = l as usize;
        let p = p as usize;
        match grid.at(l, p) {
            Some(c) => match c {
                '#' => {
                    if self.obstacles.contains(&(l, p, self.dir)) {
                        return None;
                    }
                    self.obstacles.insert((l, p, self.dir));
                    self.dir.turn_right();
                    self.walk(grid)
                }
                _ => {
                    self.line = l;
                    self.pos = p;
                    self.visited.insert((self.line, self.pos));
                    // Slick =)
                    Some(self.walk(grid)? + 1)
                }
            },
            None => Some(0),
        }
    }

    fn covered_ground(&self) -> usize {
        self.visited.len()
    }
}
