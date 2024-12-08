use aoc2024::day1;
use aoc2024::day2;
use aoc2024::day3;
use aoc2024::day4;
use aoc2024::day6;
use aoc2024::day7;
use aoc2024::day8;

macro_rules! solve_day {
    ($nb:literal) => {
        // Day $nb
        println!("Solving Day {}.", $nb);
        concat_idents!(day,$nb)::solve(concat_idents!(DAY_, $nb, _INPUT));
    };
}

const DAY_1_INPUT: &str = include_str!("../input/day1.txt");
const DAY_2_INPUT: &str = include_str!("../input/day2.txt");
const DAY_3_INPUT: &str = include_str!("../input/day3.txt");
const DAY_4_INPUT: &str = include_str!("../input/day4.txt");
const DAY_6_INPUT: &str = include_str!("../input/day6.txt");
const DAY_7_INPUT: &str = include_str!("../input/day7.txt");
const DAY_8_INPUT: &str = include_str!("../input/day8.txt");

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

    // Day 6
    println!("Solving Day 6.");
    day6::solve(DAY_6_INPUT);

    // // Day 7
    // println!("Solving Day 7.");
    // day7::solve(DAY_7_INPUT);

    // Day 8
    println!("Solving Day 8.");
    day8::solve(DAY_8_INPUT);
}
