use super::direction::Direction;
use super::point::Point;

/// Represents a point with a direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirectedPoint {
    pub point: Point,
    pub direction: Direction,
}

impl DirectedPoint {
    #[inline]
    pub fn new(x: i32, y: i32, direction: Direction) -> Self {
        Self {
            point: Point::new(x, y),
            direction,
        }
    }

    #[inline]
    pub fn new_from_xy(xy: (usize, usize), direction: Direction) -> Self {
        Self {
            point: Point::new(xy.0 as i32, xy.1 as i32),
            direction,
        }
    }

    #[inline]
    pub fn new_from_point(point: Point, direction: Direction) -> Self {
        Self {
            point: Point::new(point.x, point.y),
            direction,
        }
    }

    // Make one step in the current direction
    #[inline]
    pub fn step(&mut self) {
        self.point.step(self.direction);
    }

    /// Checks what would the point be if it was to step
    #[inline]
    pub fn peek(&self) -> Point {
        self.point.peek(self.direction)
    }
    /// Checks what would the point be if it was to step multiple times
    #[inline]
    pub fn peek_multiple(&self, steps: i32) -> Point {
        self.point.peek_multiple(self.direction, steps)
    }
}
