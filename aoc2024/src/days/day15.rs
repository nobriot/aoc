use crate::input;
use aoc_utils::direction::Direction;
use aoc_utils::grid::Grid;
use aoc_utils::moves::Moves;

use std::str::FromStr;

type Map = Grid<char>;

pub fn solve() -> (Option<usize>, Option<usize>) {
    let input = input::DAY_15_INPUT;

    let sections = input
        .split("\n\n")
        .filter(|s| !s.trim().is_empty())
        .collect::<Vec<&str>>();

    let grid: Map = Grid::from_str(sections[0]).expect("Grid should be valid. Check input");

    // Get the (multiple) move lists
    let moves: Vec<Moves> = sections[1]
        .lines()
        .map(|l| Moves::from_str(l).expect("Moves should be valid. Check input"))
        .collect();

    let part_1_total = solve_part_1(grid.clone(), &moves);
    let part_2_total = solve_part_2(&grid, &moves);

    (part_1_total, part_2_total)
}

fn solve_part_1(mut grid: Map, moves: &[Moves]) -> Option<usize> {
    for move_list in moves.iter() {
        let start = grid.find('@').expect("Grid contains @ symbol");
        apply_moves_on_grid(&mut grid, start, &move_list);
    }

    let mut part_1_count = 0;
    grid.find_all('O').for_each(|(x, y)| {
        part_1_count += 100 * y + x;
    });

    //println!("Grid after moving: \n{}", grid);
    //println!("Part 1 count: {}", part_1_count);
    Some(part_1_count)
}

fn solve_part_2(original_grid: &Map, moves: &[Moves]) -> Option<usize> {
    let mut grid: Map = expand_grid(original_grid);

    for move_list in moves.iter() {
        let start = grid.find('@').expect("Grid contains @ symbol");
        apply_moves_on_grid(&mut grid, start, &move_list);
    }
    let mut part_2_count = 0;
    grid.find_all('[').for_each(|(x, y)| {
        part_2_count += 100 * y + x;
    });

    Some(part_2_count)
}

/// Here we just hardcode objects, it will be:
/// # walls (immovable)
/// O boxes (movable)
/// . empty space
/// @ Current player
///
fn apply_moves_on_grid(
    grid: &mut Map,
    start: (usize, usize),
    moves: &Moves,
) -> Option<(usize, usize)> {
    let mut current = start;

    for m in moves.moves.iter() {
        // Increment the current player if it moved as part of the push
        if is_move_possible(grid, current, *m) {
            current = move_on_grid_unchecked(grid, current, *m);
        }
        //println!("Moving {:?} : \n{}", m, grid);
    }

    Some(current)
}

/// Moves stuff on the grid
/// Returns None if the move is not possible
/// Else the position of the new start point
fn move_on_grid(
    grid: &mut Map,
    start: (usize, usize),
    direction: Direction,
) -> Option<(usize, usize)> {
    //println!("Recursing at position {:?}", start);
    let destination = direction.move_point(start);
    match grid[destination] {
        'O' => {
            //Try to move the box ahead, then ourselves
            let _ = move_on_grid(grid, destination, direction)?;
            move_on_grid(grid, start, direction)
            //panic!("Danger");
        }
        '.' => {
            // Update all the items to move, start by moving the last
            // one by one, and then the others
            //println!("Moving to empty spot");
            grid[destination] = grid[start];
            grid[start] = '.';
            Some(destination)
        }
        _ => {
            // We hit a wall or something unknown, just move on to the next move
            // and ignore this one
            //println!("Another brick in the wall: {:?}", c);
            None
        }
    }
}

/// Moves stuff on the grid, assuming that moving is possible
fn move_on_grid_unchecked(
    grid: &mut Map,
    start: (usize, usize),
    direction: Direction,
) -> (usize, usize) {
    //println!("Recursing at position {:?}", start);
    let destination = direction.move_point(start);
    match grid[destination] {
        'O' => {
            //Try to move the box ahead, then ourselves
            move_on_grid_unchecked(grid, destination, direction);
            move_on_grid_unchecked(grid, start, direction)
            //panic!("Danger");
        }
        '[' => {
            move_on_grid_unchecked(grid, destination, direction);
            let new_start = move_on_grid_unchecked(grid, start, direction);
            if direction == Direction::Up || direction == Direction::Down {
                let right = Direction::Right.move_point(destination);
                move_on_grid_unchecked(grid, right, direction);
            }
            new_start
        }
        ']' => {
            move_on_grid_unchecked(grid, destination, direction);
            let new_start = move_on_grid_unchecked(grid, start, direction);
            if direction == Direction::Up || direction == Direction::Down {
                let right = Direction::Left.move_point(destination);
                move_on_grid_unchecked(grid, right, direction);
            }
            new_start
        }
        '.' => {
            // Update all the items to move, start by moving the last
            // one by one, and then the others
            grid[destination] = grid[start];
            grid[start] = '.';
            destination
        }
        c => {
            panic!(
                "move_on_grid_unchecked encountered unexpected obstacle {}",
                c
            );
        }
    }
}

/// Checks if it is possible to push in a given direction for a start point in the grid
///
fn is_move_possible(grid: &Map, start: (usize, usize), direction: Direction) -> bool {
    //println!("Recursing at position {:?}", start);
    let destination = direction.move_point(start);
    match grid[destination] {
        'O' => {
            //Try to move the box ahead, then ourselves
            if !is_move_possible(grid, destination, direction) {
                return false;
            }
            true
        }
        '[' => {
            // Both this side and the other side of the box must be
            // movable for it to be pushed
            if !is_move_possible(grid, destination, direction) {
                return false;
            }

            match direction {
                Direction::Up | Direction::Down => {
                    let right = Direction::Right.move_point(destination);
                    assert!(grid.point_within_bounds((right.0 as isize, right.1 as isize)));
                    if !is_move_possible(grid, right, direction) {
                        return false;
                    }
                }
                _ => {}
            }

            true
        }
        ']' => {
            // Both this side and the other side of the box must be
            // movable for it to be pushed
            if !is_move_possible(grid, destination, direction) {
                return false;
            }
            match direction {
                Direction::Up | Direction::Down => {
                    let left = Direction::Left.move_point(destination);
                    assert!(grid.point_within_bounds((left.0 as isize, left.1 as isize)));
                    if !is_move_possible(grid, left, direction) {
                        return false;
                    }
                }
                _ => {}
            }

            true
        }
        '.' => {
            // Update all the items to move, start by moving the last
            // one by one, and then the others
            //println!("Moving to empty spot");
            true
        }
        _ => {
            // We hit a wall or something unknown, just move on to the next move
            // and ignore this one
            //println!("Another brick in the wall: {:?}", c);
            false
        }
    }
}

fn expand_grid(grid: &Map) -> Map {
    // Expand the grid to large boxes
    let mut expanded = Vec::with_capacity(grid.width * 2 * grid.height);

    // There may be a way to do this without pre-allocating(like iter/zip/unzip/collect?)
    // but this works too
    grid.data.iter().for_each(|c| match *c {
        '#' => {
            expanded.push('#');
            expanded.push('#');
        }
        'O' => {
            expanded.push('[');
            expanded.push(']');
        }
        '.' => {
            expanded.push('.');
            expanded.push('.');
        }
        '@' => {
            expanded.push('@');
            expanded.push('.');
        }
        _ => panic!("Unexpected character in grid: {}", c),
    });

    Grid {
        width: grid.width * 2,
        height: grid.height,
        data: expanded,
    }
}
