use super::direction::Direction;

use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Move the point in the given direction
    /// by one step
    pub fn step(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
        }
    }

    /// Returns what would the point become if it was to
    /// step
    ///
    pub fn peek(&self, direction: Direction) -> Self {
        let mut new_point = Self {
            x: self.x,
            y: self.y,
        };

        match direction {
            Direction::Up => new_point.y -= 1,
            Direction::Right => new_point.x += 1,
            Direction::Down => new_point.y += 1,
            Direction::Left => new_point.x -= 1,
        }

        new_point
    }

    /// Returns what would the point become if it was to
    /// step multiple times
    ///
    pub fn peek_multiple(&self, direction: Direction, steps: i32) -> Self {
        let mut new_point = Self {
            x: self.x,
            y: self.y,
        };

        match direction {
            Direction::Up => new_point.y -= steps,
            Direction::Right => new_point.x += steps,
            Direction::Down => new_point.y += steps,
            Direction::Left => new_point.x -= steps,
        }

        new_point
    }

    /// Returns the point coordinates as a usize (x,y) tuple
    #[inline]
    pub fn as_usize_tuple(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }

    /// Returns the point coordinates as a i32 (x,y) tuple
    #[inline]
    pub fn as_i32_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}
