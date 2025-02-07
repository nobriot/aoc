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

    let part_1_total = solve_part_1(&sections);
    let part_2_total = solve_part_2(input);

    (part_1_total, part_2_total)
}

fn solve_part_1(sections: &Vec<&str>) -> Option<usize> {
    // Build the grid
    let mut grid: Map = Grid::from_str(sections[0]).expect("Grid should be valid. Check input");

    // Get the (multiple) move lists
    let moves: Vec<Moves> = sections[1]
        .lines()
        .map(|l| Moves::from_str(l).expect("Moves should be valid. Check input"))
        .collect();

    // println!("We have {} MoveLists", moves.len());

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

fn solve_part_2(input: &str) -> Option<usize> {
    None
}

/// Here we just hardcode objects, it will be:
/// # walls (immovable)
/// O boxes (movable)
/// . empty space
/// @ Current player
///
pub fn apply_moves_on_grid(
    grid: &mut Map,
    start: (usize, usize),
    moves: &Moves,
) -> Option<(usize, usize)> {
    let mut current = start;

    for m in moves.moves.iter() {
        // Increment the current player if it moved as part of the push
        if let Some(new_position) = move_on_grid(grid, current, *m) {
            current = new_position;
        }
    }

    Some(current)
}

/// Moves stuff on the grid
/// Returns None if the move is not possible
/// Else the position of the new start point
pub fn move_on_grid(
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
