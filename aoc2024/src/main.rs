use aoc2024::day1;
use aoc2024::day2;
use aoc2024::day3;
use aoc2024::day4;

const DAY_1_INPUT: &str = include_str!("../input/day1.txt");
const DAY_2_INPUT: &str = include_str!("../input/day2.txt");
const DAY_3_INPUT: &str = include_str!("../input/day3.txt");
const DAY_4_INPUT: &str = include_str!("../input/day4.txt");

fn main() {
    // Day 1
    println!("Solving Day 1.");
    day1::solve(DAY_1_INPUT);

    // Day 2
    println!("Solving Day 2.");
    day2::solve(DAY_2_INPUT);

    // Day 3
    println!("Solving Day 3.");
    day3::solve(DAY_3_INPUT);

    // Day 4
    println!("Solving Day 4.");
    day4::solve(DAY_4_INPUT);
}
