use std::ops::Index;
use std::str::FromStr;

/// We often need to make a 2 dimensional grid of some sort.
/// This is a helper struct to make that easier.
/// Most of the time each row has the same length, so we
/// store the grid here as a 1-dimension vector of data
///
#[derive(Debug)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    data: Vec<T>,
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
    pub fn new(&self, width: usize, height: usize, value: T) -> Self {
        Self {
            width,
            height,
            data: vec![value; width * height],
        }
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

    /// Checks if a coordinate tuple is within bounds of the grid
    #[inline]
    pub fn within_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && (x as usize) < self.width && y >= 0 && (y as usize) < self.height
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
/// Using a `isize` tuple to lookup a value in the grid
///
impl<T> Index<(i32, i32)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (i32, i32)) -> &Self::Output {
        &self.data[y as usize * self.width + x as usize]
    }
}

/// Derives a string into a char grid
/// Could probably be more efficient with u8
/// .... Maybe later
impl FromStr for Grid<char> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().nth(0).ok_or(())?.len();
        let data: Vec<char> = s.lines().map(|l| l.chars()).flatten().collect();
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
        let data: Vec<u8> = s.lines().map(|l| l.bytes()).flatten().collect();
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
        assert!(grid.within_bounds(0, 0));
        assert!(!grid.within_bounds(-1, 0));
        assert!(!grid.within_bounds(0, -1));
        assert!(grid.within_bounds((grid.width - 1) as isize, 0));
        assert!(!grid.within_bounds((grid.width) as isize, 0));
        assert!(grid.within_bounds(0, (grid.height - 1) as isize));
        assert!(!grid.within_bounds(0, (grid.height) as isize));
    }
}
