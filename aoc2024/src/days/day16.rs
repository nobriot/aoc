use crate::input;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve() -> (Option<usize>, Option<usize>) {
    let input = input::DAY_16_INPUT;
    let grid = Grid::from_str(input);

    let part_1_total = solve_part_1(&grid);
    // println!("Part 1 Result: {part_1_total}");

    let part_2_total = solve_part_2(&grid);
    // println!("Part 2 Result: {part_2_total}");
    //
    (part_1_total, part_2_total)
}

fn solve_part_1(grid: &Grid) -> Option<usize> {
    None
}

fn solve_part_2(grid: &Grid) -> Option<usize> {
    None
}

#[derive(Debug)]
struct Grid {
    data: Vec<Vec<char>>,
}

impl Grid {
    const FORWARD: usize = 1;
    const TURN: usize = 1000;

    fn from_str(input: &str) -> Self {
        let data: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
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
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_clockwise(&mut self) {
        match self {
            Direction::Up => *self = Direction::Right,
            Direction::Right => *self = Direction::Down,
            Direction::Down => *self = Direction::Left,
            Direction::Left => *self = Direction::Up,
        }
    }
    fn turn_counterclockwise(&mut self) {
        match self {
            Direction::Up => *self = Direction::Left,
            Direction::Right => *self = Direction::Up,
            Direction::Down => *self = Direction::Right,
            Direction::Left => *self = Direction::Down,
        }
    }
}

#[derive(Debug)]
struct Reindeer {
    line: usize,
    pos: usize,
    dir: Direction,
    /// Keeping track of visited positions with their associated cost
    visited: HashMap<(usize, usize), usize>,
}

impl Reindeer {
    fn from_grid(grid: &Grid) -> Option<Self> {
        let opt = grid.find_char('S');
        if opt.is_some() {
            let (line, pos) = opt.unwrap();
            let mut visited = HashMap::new();
            visited.insert((line, pos), 0);
            return Some(Self {
                line,
                pos,
                dir: Direction::Right,
                visited,
            });
        }
        None
    }

    // Maps the grid with costs until finding the E tile
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
        todo!();
        //         match grid.at(l, p) {
        //             Some(c) => match c {
        //                 '#' => {
        //                     if self.obstacles.contains(&(l, p, self.dir)) {
        //                         return None;
        //                     }
        //                     self.obstacles.insert((l, p, self.dir));
        //                     // self.dir.turn_right();
        //                     self.walk(grid)
        //                 }
        //                 _ => {
        //                     self.line = l;
        //                     self.pos = p;
        //                     self.visited.insert((self.line, self.pos));
        //                     // Slick =)
        //                     Some(self.walk(grid)? + 1)
        //                 }
        //             },
        //             None => Some(0),
        //         }
    }

    fn covered_ground(&self) -> usize {
        self.visited.len()
    }
}
