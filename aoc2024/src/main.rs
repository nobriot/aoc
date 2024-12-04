use aoc2024::day1;
use aoc2024::day2;
use aoc2024::day3;

static SOURCE_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn main() {
    // Day 1
    println!("Solving Day 1.");
    day1::solve(format!("{}/input/day1.txt", SOURCE_DIR).as_str());

    // Day 2
    println!("Solving Day 2.");
    day2::solve(format!("{}/input/day2.txt", SOURCE_DIR).as_str());

    // Day 3
    println!("Solving Day 3.");
    day3::solve(format!("{}/input/day3.txt", SOURCE_DIR).as_str());
}
