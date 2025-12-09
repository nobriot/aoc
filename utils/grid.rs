use std::ops::{Index, IndexMut};
use std::str::FromStr;

use super::point::Point;

/// We often need to make a 2 dimensional grid of some sort.
/// This is a helper struct to make that easier.
/// Most of the time each row has the same length, so we
/// store the grid here as a 1-dimension vector of data
///
#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<T>,
}

impl<T> Grid<T> {
    /// Indicates the total length of the grid (width * height)
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Indicates if the grid has zero-size
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl<T: Copy> Grid<T> {
    /// Create a new grid with the indicated dimensions, and filled with the default value
    ///
    pub fn new(width: usize, height: usize, value: T) -> Self {
        Self {
            width,
            height,
            data: vec![value; width * height],
        }
    }

    /// Creates a new grid from a slice of data
    /// The slice must be at least width * height long
    ///
    pub fn from_slice(width: usize, height: usize, data: &[T]) -> Self {
        assert!(
            width * height >= data.len(),
            "Data is too small for height/width"
        );
        Self {
            width,
            height,
            data: data[..height * width].to_vec(),
        }
    }

    /// Checks if a couple of coordinates is within bounds of the grid
    #[inline]
    pub fn xy_within_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && (x as usize) < self.width && y >= 0 && (y as usize) < self.height
    }

    pub fn neighbors(&self, point: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        let point = Point::new(point.0 as i32, point.1 as i32);
        let mut neighbors = Vec::with_capacity(8);
        for &offset in &[
            Point::new(-1, -1),
            Point::new(-1, 0),
            Point::new(-1, 1),
            Point::new(0, -1),
            Point::new(0, 1),
            Point::new(1, -1),
            Point::new(1, 0),
            Point::new(1, 1),
        ] {
            let neighbor_point = point + offset;
            if self.point_within_bounds((neighbor_point.x as isize, neighbor_point.y as isize)) {
                neighbors.push((neighbor_point.x as usize, neighbor_point.y as usize));
            }
        }

        neighbors.into_iter()
    }

    /// Checks if a coordinate tuple is within bounds of the grid
    #[inline]
    pub fn point_within_bounds(&self, point: (isize, isize)) -> bool {
        point.0 >= 0
            && (point.0 as usize) < self.width
            && point.1 >= 0
            && (point.1 as usize) < self.height
    }
}

impl<T: Copy + PartialEq> Grid<T> {
    /// Finds the first element in the grid that is equal
    /// to the value passed in parameter
    ///
    /// Returns value (y, x), y being line number, x the col number
    ///
    pub fn find(&self, value: T) -> Option<(usize, usize)> {
        self.data
            .iter()
            .position(|&x| x == value)
            .map(|i| (i % self.width, i / self.width))
    }

    /// Returns an iterator over the the values in the grid
    /// that are equal to the value passed in parameter
    ///
    /// Return value (x, y), coordinate of elements equal to the value
    ///
    pub fn find_all(&self, value: T) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.data
            .iter()
            .enumerate()
            .filter(move |(_, &v)| v == value)
            .map(move |(i, _)| (i % self.width, i / self.width))
    }
}

///
/// Printing the data contained in a grid.
///
impl<T> std::fmt::Display for Grid<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.data[y * self.width + x])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: std::fmt::Display + Copy> Grid<Option<T>> {
    /// Create a new grid with the indicated dimensions, and filled with the default value
    ///
    pub fn display_grid_with_options(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.data[y * self.width + x] {
                    Some(value) => print!("{:>5}", value),
                    None => print!("  .  "),
                }
            }
            println!();
        }
    }
}

impl Grid<bool> {
    /// Displays a grid with bool values
    ///
    pub fn display_grid_with_bool(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.data[y * self.width + x] {
                    true => print!("x"),
                    false => print!("."),
                }
            }
            println!();
        }
    }
}

///
/// Using a `usize` tuple to lookup a value in the grid
///
impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    /// x is based on the width and y on the height
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.data[y * self.width + x]
    }
}

///
/// Using a `isize` tuple to lookup a value in the grid
///
impl<T> Index<(isize, isize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (isize, isize)) -> &Self::Output {
        &self.data[y as usize * self.width + x as usize]
    }
}

///
/// Using a `i32` tuple to lookup a value in the grid
///
impl<T> Index<(i32, i32)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (i32, i32)) -> &Self::Output {
        &self.data[y as usize * self.width + x as usize]
    }
}

///
/// Using a Point (x,y) to lookup a value in the grid
///
impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, point: Point) -> &Self::Output {
        &self.data[point.y as usize * self.width + point.x as usize]
    }
}

///
/// Using a `usize` tuple to set a value in the grid
///
impl<T> IndexMut<(usize, usize)> for Grid<T> {
    /// x is based on the width and y on the height
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.data[y * self.width + x]
    }
}

///
/// Using a `isize` tuple to set a value in the grid
///
impl<T> IndexMut<(isize, isize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (isize, isize)) -> &mut Self::Output {
        &mut self.data[y as usize * self.width + x as usize]
    }
}

///
/// Using a `i32` tuple to set a value in the grid
///
impl<T> IndexMut<(i32, i32)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (i32, i32)) -> &mut Self::Output {
        &mut self.data[y as usize * self.width + x as usize]
    }
}

///
/// Using a Point (x,y) to set a value in the grid
///
impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        &mut self.data[point.y as usize * self.width + point.x as usize]
    }
}

/// Derives a string into a char grid
/// Could probably be more efficient with u8
/// .... Maybe later
impl FromStr for Grid<char> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().filter(|line| !line.trim().is_empty()).count();
        let width = s.lines().nth(0).ok_or(())?.len();
        let data: Vec<char> = s.lines().flat_map(|l| l.chars()).collect();

        if data.len() != width * height {
            // eprintln!( "Error building grid: len {} vs width {} height {}", data.len(), width, height );
            return Err(());
        }

        Ok(Self {
            width,
            height,
            data,
        })
    }
}

/// Derives a string into a u8 / byte
/// grid
impl FromStr for Grid<u8> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().nth(0).ok_or(())?.len();
        let data: Vec<u8> = s.lines().flat_map(|l| l.bytes()).collect();
        Ok(Self {
            width,
            height,
            data,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid() {
        let input = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########
"#;

        let grid: Grid<char> = Grid::from_str(input).unwrap();

        // We expect the first line to be #
        for x in 0..grid.width {
            assert_eq!(grid[(x, 0)], '#', "Unexpected character at {}, 0", x);
        }

        assert_eq!(
            grid.height,
            input.lines().count(),
            "Unexpected height, actual {}, expected {}",
            grid.height,
            input.lines().count()
        );
        assert_eq!(grid[(0, 4)], '#');
        assert_eq!(grid[(1, 4)], '.');
        assert_eq!(grid[(2, 4)], '.');
        assert_eq!(grid[(3, 4)], 'O');
        assert_eq!(grid[(4, 4)], '@');
        assert_eq!(grid[(5, 4)], '.');

        // Check the find function:
        assert_eq!(grid.find('@'), Some((4, 4)), "Unexpected position for @");
        assert_eq!(grid.find('X'), None, "X is not present in the grid");

        // Check bounds
        assert!(grid.xy_within_bounds(0, 0));
        assert!(!grid.xy_within_bounds(-1, 0));
        assert!(!grid.xy_within_bounds(0, -1));
        assert!(grid.xy_within_bounds((grid.width - 1) as isize, 0));
        assert!(!grid.xy_within_bounds((grid.width) as isize, 0));
        assert!(grid.xy_within_bounds(0, (grid.height - 1) as isize));
        assert!(!grid.xy_within_bounds(0, (grid.height) as isize));

        // Check some neighbors
        assert_eq!(3, grid.neighbors((0, 0)).count());
        assert_eq!(8, grid.neighbors((1, 1)).count());
    }

    #[test]
    fn test_grid_find_all() {
        let input = "..x\n.x.\n..x";

        let grid: Grid<char> = Grid::from_str(input).unwrap();

        let mut iter = grid.find_all('x');
        assert_eq!(iter.next(), Some((2, 0)));
        assert_eq!(iter.next(), Some((1, 1)));
        assert_eq!(iter.next(), Some((2, 2)));
        assert_eq!(iter.next(), None);

        let mut iter = grid.find_all('@');
        assert_eq!(iter.next(), None);
    }
}
