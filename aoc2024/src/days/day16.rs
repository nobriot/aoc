use crate::input;

use aoc_utils::DirectedPoint;
use aoc_utils::Direction;
use aoc_utils::Grid;
use std::str::FromStr;

type Map = Grid<char>;

#[derive(Debug, Clone)]
struct Reindeer {
    pub p: DirectedPoint,
    pub score: usize,
    /// If the reindeer is walking on already visited paths, it should not turn around
    /// unless it suddenly finds some un-visited tiles
    pub no_more_turns: bool,
    /// This is ised mostly to prevent reindeer from turning around witout moving. We allow turns
    /// at the start, (set has_moved to true), but not after each turn (else it just spins on
    /// itself).
    pub has_moved: bool,
}

pub fn solve() -> (Option<usize>, Option<usize>) {
    let input = input::DAY_16_INPUT;
    let grid: Map = Grid::from_str(input).expect("Grid should be valid");

    // Here we have to use the part 1 visited grid to solve part 2, so we solve it "here"
    let visited = find_best_path(&grid);
    let start = grid.find('S').expect("S should be in input grid");
    let goal = grid.find('E').expect("E should be in input grid");
    let best_sits = count_best_sits(&visited, start, goal);

    let part_1_total = Some(visited[goal].expect("Goal should be reached"));
    let part_2_total = Some(best_sits);

    (part_1_total, part_2_total)
}

/// Takes the grid and find the best path from S to E
/// Returns a map of visited tiles with the scores
///
fn find_best_path(grid: &Map) -> Grid<Option<usize>> {
    let start_position = grid.find('S').expect("S should be in input grid");
    // println!("Start position: {:?}", start_position);
    let goal = grid.find('E').expect("E should be in input grid");
    // println!("Goal: {:?}", goal);

    // The reindeer is facing East
    let mut reindeer = Reindeer {
        p: DirectedPoint::new_from_xy(start_position, Direction::Right),
        score: 0,
        no_more_turns: false,
        has_moved: true,
    };

    let mut reindeer_paths = vec![reindeer.clone()];
    let mut visited: Grid<Option<usize>> = Grid::new(grid.width, grid.height, None);
    visited[start_position] = Some(0);

    // We need to keep running over visited ground because we may get further with a
    // lower score in this case:
    //
    // ##^##
    // >>^..
    // ##^##
    //
    // Where the upwards arrow turning right may lead to a more expensive
    // score than the arrow going right, even if arrow going right arrived after.
    // Though we should prevent such reindeer path to keep turning around afterwards
    // if it is stepping on already visited ground and has not been the "fastest path anywhere"
    let mut loop_count = 0;
    while visited[goal].is_none() {
        loop_count += 1;
        // Check if we should queue turns
        if !reindeer.no_more_turns && reindeer.has_moved {
            for other_direction in reindeer.p.direction.perpendiculars() {
                let other_destination = reindeer.p.point.peek(other_direction);
                if grid[other_destination] != '#' {
                    let new_reindeer = Reindeer {
                        p: DirectedPoint::new_from_point(reindeer.p.point, other_direction),
                        score: reindeer.score + 1000,
                        no_more_turns: visited[other_destination].is_some(),
                        has_moved: false,
                    };
                    reindeer_paths.push(new_reindeer);
                }
            }
        }

        let step_forward = reindeer.p.peek();
        //println!("Step forward {:?}", step_forward);
        if grid[step_forward] != '#' {
            reindeer.p.step();
            reindeer.score += 1;
            reindeer.has_moved = true;

            if let Some(previous_score) = visited[step_forward] {
                reindeer.no_more_turns = true;
                if reindeer.score < previous_score {
                    visited[step_forward] = Some(reindeer.score);
                    reindeer.no_more_turns = false;
                }
            } else {
                visited[reindeer.p.point] = Some(reindeer.score);
                reindeer.no_more_turns = false;
            }

            // Put the current reindeer back in the queue
            reindeer_paths.push(reindeer);
            // println!("Reindeer re-entering: {:?}", reindeer);
        }

        // Find out which reindeer has the lowest score
        let mut cheapest_index = 0;
        // Indexing here will panic if we did not reach the goal and have no more
        // reindeer paths to explore. Seems fine
        let mut cheapest = reindeer_paths[cheapest_index].score;

        reindeer_paths.iter().enumerate().for_each(|(i, r)| {
            if r.score < cheapest {
                cheapest = r.score;
                cheapest_index = i;
            }
        });
        reindeer = reindeer_paths.swap_remove(cheapest_index);

        // println!("Reindeer paths length: {:?}", reindeer_paths.len());
        // println!("Reindeer paths:");
        // for r in &reindeer_paths {
        // println!("{:?}", r);
        // }
        // println!("");
        // visited.display_grid_with_options();
    }

    visited
}

fn count_best_sits(
    visited: &Grid<Option<usize>>,
    start: (usize, usize),
    goal: (usize, usize),
) -> usize {
    // We start from the goal, and keep following trails that have a decrement by 1 (or 1000) til the start.

    let mut paths: Grid<bool> = Grid::new(visited.width, visited.height, false);
    paths[goal] = true;
    paths[start] = true;

    let mut reindeer_paths: Vec<Reindeer> = Vec::new();
    for direction in Direction::all() {
        let new_reindeer = Reindeer {
            p: DirectedPoint::new_from_xy((goal.0, goal.1), direction),
            score: visited[goal].unwrap(),
            no_more_turns: false,
            has_moved: false,
        };
        reindeer_paths.push(new_reindeer);
    }

    while !reindeer_paths.is_empty() {
        // Safe as vector is non-empty
        let mut current = reindeer_paths.pop().unwrap();

        // If we arrived stop searching
        if current.p.point.as_usize_tuple() == start {
            continue;
        }

        // Queue turns
        if current.has_moved && !current.no_more_turns {
            for other_direction in current.p.direction.perpendiculars() {
                let other_destination = current.p.point.peek(other_direction);
                if visited[other_destination].is_some() {
                    let new_reindeer = Reindeer {
                        p: DirectedPoint::new_from_point(current.p.point, other_direction),
                        score: current.score - 1000,
                        no_more_turns: false,
                        has_moved: false,
                    };
                    reindeer_paths.push(new_reindeer);
                }
            }
        }

        let step_forward = current.p.peek();
        if let Some(best_score) = visited[step_forward] {
            current.p.step();
            current.score -= 1;
            current.has_moved = true;

            // Here if the best score is even better than the deer, it means we are about to
            // turn. It will regulate after turning
            // The current.score is an upper bound for making sure we are on an optimal path
            if current.score >= best_score {
                paths[current.p.point] = true;
                current.no_more_turns = false;
                reindeer_paths.push(current);
            }
        }

        //paths.display_grid_with_bool();
    }

    paths.data.iter().filter(|&&p| p).count()
}
