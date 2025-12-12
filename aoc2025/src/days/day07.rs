use std::{collections::HashMap, collections::HashSet, str::FromStr};

use crate::input;
use aoc_utils::Grid;

pub fn solve() -> (Option<usize>, Option<usize>) {
    let input = input::DAY_7_INPUT;

    let grid: Grid<char> = Grid::from_str(input).expect("Grid-like input");

    let part_1_total = solve_part_1(&grid);
    let part_2_total = solve_part_2(&grid);

    (part_1_total, part_2_total)
}

fn solve_part_1(grid: &Grid<char>) -> Option<usize> {
    let start = grid.find('S').expect("S in input");
    let mut beams = HashSet::new();
    beams.insert(start);
    let mut split_count = 0;

    'main: loop {
        let mut new_beams = HashSet::new();
        for beam in beams {
            let down = (beam.0, beam.1 + 1);

            if !grid.usize_point_within_bounds(down) {
                break 'main;
            }
            if grid[down] == '^' {
                let right = (down.0 + 1, down.1);
                if grid.usize_point_within_bounds(right) {
                    new_beams.insert(right);
                }
                let left = (down.0 - 1, down.1);
                if grid.usize_point_within_bounds(left) {
                    new_beams.insert(left);
                }
                split_count += 1;
            } else {
                new_beams.insert(down);
            }
        }

        beams = new_beams;
    }

    Some(split_count)
}

fn solve_part_2(grid: &Grid<char>) -> Option<usize> {
    let start = grid.find('S').expect("S in input");
    let mut beams = HashSet::new();
    // Stores how many paths could have lead to a given beam
    let mut beams_power = HashMap::new();
    beams.insert(start);
    beams_power.insert(start, 1);

    'main: loop {
        let mut new_beams = HashSet::new();
        for &beam in &beams {
            let down = (beam.0, beam.1 + 1);
            if !grid.usize_point_within_bounds(down) {
                break 'main;
            }

            let parent_power = *beams_power.get(&beam).unwrap();
            assert!(parent_power > 0);

            if grid[down] == '^' {
                let right = (down.0 + 1, down.1);
                if grid.usize_point_within_bounds(right) {
                    let power = if let Some(&p) = beams_power.get(&right) {
                        p
                    } else {
                        0
                    };
                    new_beams.insert(right);
                    beams_power.insert(right, parent_power + power);
                }
                let left = (down.0 - 1, down.1);
                if grid.usize_point_within_bounds(left) {
                    let power = if let Some(&p) = beams_power.get(&left) {
                        p
                    } else {
                        0
                    };
                    new_beams.insert(left);
                    beams_power.insert(left, parent_power + power);
                }
            } else {
                let power = if let Some(&p) = beams_power.get(&down) {
                    p
                } else {
                    0
                };
                new_beams.insert(down);
                beams_power.insert(down, parent_power + power);
            }
        }

        beams = new_beams;
        beams_power.retain(|&k, _| beams.contains(&k));
    }

    let mut multiverses = 0;
    for beam in beams {
        multiverses += beams_power.get(&beam).unwrap();
    }

    Some(multiverses)
}
