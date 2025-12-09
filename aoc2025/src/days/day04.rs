use std::str::FromStr;

use crate::input;
use aoc_utils::Grid;

pub fn solve() -> (Option<usize>, Option<usize>) {
    let input = input::DAY_4_INPUT;

    let mut grid: Grid<char> = Grid::from_str(input).expect("valid grid");

    let part_1_total = solve_part_1(&grid);
    let part_2_total = solve_part_2(&mut grid);

    (part_1_total, part_2_total)
}

fn solve_part_1(grid: &Grid<char>) -> Option<usize> {
    let mut forklift_rolls = 0;
    for roll in grid.find_all('@') {
        // Find adjacent
        let mut roll_count = 0;
        for neighbor in grid.neighbors(roll) {
            if grid[neighbor] == '@' {
                roll_count += 1;
            }
        }

        if roll_count < 4 {
            forklift_rolls += 1;
        }
    }

    Some(forklift_rolls)
}

fn solve_part_2(grid: &mut Grid<char>) -> Option<usize> {
    // There is probably a way to calculate which rolls are going to stay
    let mut roll_removed = true;
    let mut total_removed = 0;
    let mut to_remove = Vec::new();

    while roll_removed {
        to_remove.clear();
        roll_removed = false;
        for roll in grid.find_all('@') {
            // Find adjacent
            let mut roll_count = 0;
            for neighbor in grid.neighbors(roll) {
                if grid[neighbor] == '@' {
                    roll_count += 1;
                }
            }

            if roll_count < 4 {
                to_remove.push(roll);
                roll_removed = true;
                total_removed += 1;
            }
        }

        for &roll in &to_remove {
            grid[roll] = '.'
        }
    }

    Some(total_removed)
}
