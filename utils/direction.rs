use super::grid::Grid;
use std::str::FromStr;

/// Direction, used in particular for 2-d navigation puzzles
///
/// Each direction is represented with <, >, ^, v
/// This is used to move around the grid, or e.g.
/// describe movement in a game.
///
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    /// Turn the direction clockwise.
    fn turn_clockwise(&mut self) {
        match self {
            Direction::Up => *self = Direction::Right,
            Direction::Right => *self = Direction::Down,
            Direction::Down => *self = Direction::Left,
            Direction::Left => *self = Direction::Up,
        }
    }

    /// Turn the direction counter-clockwise.
    fn turn_counterclockwise(&mut self) {
        match self {
            Direction::Up => *self = Direction::Left,
            Direction::Right => *self = Direction::Up,
            Direction::Down => *self = Direction::Right,
            Direction::Left => *self = Direction::Down,
        }
    }

    /// Updates x and y coordinates if we take one steop in the current
    /// direction
    /// Returns new (x, y) coordinates
    pub fn move_xy(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Direction::Up => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
        }
    }

    /// Updates a point with (x, y) if we take one steop in the current
    /// direction
    /// Returns new point with (x, y) coordinates
    pub fn move_point(&self, point: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up => (point.0, point.1 - 1),
            Direction::Right => (point.0 + 1, point.1),
            Direction::Down => (point.0, point.1 + 1),
            Direction::Left => (point.0 - 1, point.1),
        }
    }

    /// Creates a direction from a char
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '^' => Some(Direction::Up),
            '>' => Some(Direction::Right),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            _ => None,
        }
    }
}

///
/// Printing direction with arrows
///
impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<',
        };
        write!(f, "{}", c)?;
        Ok(())
    }
}
