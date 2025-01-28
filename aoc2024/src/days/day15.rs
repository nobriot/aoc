use crate::input;
use aoc_utils::direction::Direction;
use aoc_utils::grid::Grid;

pub fn solve() -> (Option<usize>, Option<usize>) {
    let input = input::DAY_15_INPUT;

    let part_1_total = solve_part_1(input);
    // println!("Part 1 Result: {part_1_total}");

    let part_2_total = solve_part_2(input);
    // println!("Part 2 Result: {part_2_total}");
    //
    (part_1_total, part_2_total)
}

fn solve_part_1(input: &str) -> Option<usize> {
    None
}

fn solve_part_2(input: &str) -> Option<usize> {
    None
}

struct MoveSequence {
    moves: Vec<Direction>,
}

impl std::str::FromStr for MoveSequence {
    type Err = ();

    /// Each char is representing a move v , < , > , ^
    /// Error on any other char
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let moves = s
            .chars()
            .map(|c| Direction::from_char(c).unwrap())
            .collect();

        Ok(MoveSequence { moves })
    }
}
