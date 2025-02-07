use crate::input;
use std::cmp::Ordering;

pub fn solve() -> (Option<usize>, Option<usize>) {
    let input = input::DAY_14_INPUT;

    let part_1_total = solve_part_1(input);
    let part_2_total = solve_part_2(input);

    (part_1_total, part_2_total)
}

fn solve_part_1(input: &str) -> Option<usize> {
    let mut tiles = input.parse::<Tiles>().expect("Invalid tiles format");

    for _ in 0..100 {
        tiles.increment();
        // Okay now I realize that incrementing 100 times is just the same as a single operation
        // adding dx*100 and dy*100
        // Too late I don't care
    }

    //println!("Tiles after increment : {:?}", tiles);

    Some(tiles.calculate_quadrants())
}

/// Okay here I guess we will just brute force, iterate and check
/// the 2 conditions : is there just 1 robot per tile and is there symmetry
/// in the figure.
///
fn solve_part_2(input: &str) -> Option<usize> {
    let mut tiles = input.parse::<Tiles>().expect("Invalid tiles format");
    let mut i = 0;

    loop {
        tiles.increment();
        i += 1;

        if !tiles.has_one_robot_per_tile() {
            continue;
        }

        return Some(i);

        // if tiles.has_symmetry() {
        //     return Some(i);
        // }

        // // As I understand, the robot will fall back in the same position was WIDTH*HEIGHT
        // // iterations
        // //
        // // Just safeguard here against an infinite loop
        // if i > Tiles::DEFAULT_WIDTH * Tiles::DEFAULT_HEIGHT {
        //     eprintln!("Hmm does not look like we are converging");
        //     break;
        // }
    }
    None
}

#[derive(Debug)]
struct Tiles {
    /// 2-dimensional array with robots on each tile
    width: usize,
    height: usize,
    robots: Vec<Robot>,
}

impl Tiles {
    const DEFAULT_WIDTH: usize = 101;
    const DEFAULT_HEIGHT: usize = 103;

    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            robots: Vec::new(),
        }
    }

    pub fn add(&mut self, robot: Robot) {
        debug_assert!(robot.x < self.width);
        debug_assert!(robot.y < self.height);
        self.robots.push(robot);
    }

    pub fn increment(&mut self) {
        for robot in self.robots.iter_mut() {
            let mut new_x: isize = robot.x as isize + robot.dx;
            while new_x < 0 {
                new_x += self.width as isize;
            }
            if new_x >= self.width as isize {
                new_x %= self.width as isize;
            }
            robot.x = new_x as usize;

            let mut new_y: isize = robot.y as isize + robot.dy;
            while new_y < 0 {
                new_y += self.height as isize;
            }
            if new_y >= self.height as isize {
                new_y %= self.height as isize;
            }
            robot.y = new_y as usize;
        }
    }

    pub fn calculate_quadrants(&mut self) -> usize {
        let h = self.height / 2;
        let v = self.width / 2;
        let mut quadrants: [usize; 4] = [0, 0, 0, 0];

        for robot in self.robots.iter() {
            match (robot.x.cmp(&v), robot.y.cmp(&h)) {
                (Ordering::Less, Ordering::Less) => quadrants[0] += 1,
                (Ordering::Less, Ordering::Greater) => quadrants[1] += 1,
                (Ordering::Greater, Ordering::Less) => quadrants[2] += 1,
                (Ordering::Greater, Ordering::Greater) => quadrants[3] += 1,
                (_, _) => {}
            }
        }

        quadrants.iter().product()
    }

    // Hmm turns out not to be needed
    //     /// Check the leftmost robot and the rightmost robot, find the vertical symmetry
    //     /// OR
    //     /// Check the topmost robot and the bottommost robot, find the horizontal symmetry
    //     pub fn has_symmetry(&self) -> bool {
    //         // Find the bounds
    //         let mut x_min = self.width;
    //         let mut y_min = self.width;
    //         let mut x_max = 0;
    //         let mut y_max = 0;
    //
    //         let mut tiles = vec![vec![false; self.height]; self.width];
    //         for robot in self.robots.iter() {
    //             x_min = x_min.min(robot.x);
    //             y_min = y_min.min(robot.y);
    //             x_max = x_max.max(robot.x);
    //             y_max = y_max.max(robot.y);
    //             tiles[robot.x][robot.y] = true;
    //         }
    //
    //         // span = 5 , center = on a point:  x x v x x
    //         // span = 6 , center = between:     x x x v x x x
    //         let v: f32 = (x_min + x_max) as f32 / 2.0;
    //         let h: f32 = (y_min + y_max) as f32 / 2.0;
    //
    //         for robot in self.robots.iter() {
    //             let x_offset = (robot.x as f32 - v);
    //             let y_offset = (robot.y as f32 - h);
    //
    //             let (x, y) = ((v + x_offset) as usize, (h + y_offset) as usize);
    //
    //             if !tiles[x][y] {
    //                 return false;
    //             }
    //         }
    //
    //         false
    //     }

    pub fn has_one_robot_per_tile(&self) -> bool {
        let mut tiles = vec![0; self.width * self.height];

        for robot in self.robots.iter() {
            let index = robot.y * self.width + robot.x;
            tiles[index] += 1;
        }

        let result = tiles.iter().all(|&x| x <= 1);

        if result {
            println!("Found config with 1 robot per tile!");
            for i in 0..self.height {
                for j in 0..self.width {
                    print!("{:?}", tiles[i * self.width + j]);
                }
                println!();
            }
        }
        result
    }
}

impl std::str::FromStr for Tiles {
    type Err = ();

    /// Each line of the input should represent a robot
    ///
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tiles = Tiles::new(Tiles::DEFAULT_WIDTH, Tiles::DEFAULT_HEIGHT);

        for line in s.lines() {
            let robot = line.parse();
            if robot.is_err() {
                continue;
            }
            tiles.add(robot?);
        }
        Ok(tiles)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Robot {
    /// Iniial x-coordinate
    x: usize,
    /// Initial y-coordinate
    y: usize,
    /// x-axis velocity
    dx: isize,
    /// y-axis velocity
    dy: isize,
}

impl Robot {
    pub fn new(x: usize, y: usize, dx: isize, dy: isize) -> Self {
        Self { x, y, dx, dy }
    }

    pub fn inside_quadrant(&self, x: usize, y: usize, width: usize, height: usize) -> bool {
        self.x >= x && self.x < x + width && self.y >= y && self.y < y + height
    }
}

impl std::str::FromStr for Robot {
    type Err = ();

    /// The format is 'p=x,y v=dx,dy'
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let coordinates = parts.next();
        if coordinates.is_none() {
            return Err(());
        }
        let coordinates = coordinates.unwrap();

        let coordinates = coordinates
            .strip_prefix("p=")
            .expect("Could not remove p=")
            .split_once(",")
            .expect("Missing comma between x and y");
        let x = coordinates.0.parse().expect("Invalid x-coordinate");
        let y = coordinates.1.parse().expect("Invalid x-coordinate");

        let velocity = parts.next().expect("No velocity found");
        let velocity = velocity
            .strip_prefix("v=")
            .expect("Could not remove v=")
            .split_once(",")
            .expect("Missing comma between dx and dy");
        let dx = velocity.0.parse().expect("Invalid dx-coordinate");
        let dy = velocity.1.parse().expect("Invalid dy-coordinate");

        Ok(Self::new(x, y, dx, dy))
    }
}
