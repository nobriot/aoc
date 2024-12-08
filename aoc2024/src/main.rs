use clap::Parser;

use aoc2024::day1;
use aoc2024::day2;
use aoc2024::day3;
use aoc2024::day4;
use aoc2024::day5;
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
const DAY_5_INPUT: &str = include_str!("../input/day5.txt");
const DAY_6_INPUT: &str = include_str!("../input/day6.txt");
const DAY_7_INPUT: &str = include_str!("../input/day7.txt");
const DAY_8_INPUT: &str = include_str!("../input/day8.txt");

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    all: bool,

    #[arg(long)]
    day1: bool,
    #[arg(long)]
    day2: bool,
    #[arg(long)]
    day3: bool,
    #[arg(long)]
    day4: bool,
    #[arg(long)]
    day5: bool,
    #[arg(long)]
    day6: bool,
    #[arg(long)]
    day7: bool,
    #[arg(long)]
    day8: bool,
}

fn main() {
    let args = Args::parse();

    // Day 1
    if args.all || args.day1 {
        println!("Solving Day 1.");
        day1::solve(DAY_1_INPUT);
    }

    // Day 2
    if args.all || args.day2 {
        println!("Solving Day 2.");
        day2::solve(DAY_2_INPUT);
    }

    // Day 3
    if args.all || args.day3 {
        println!("Solving Day 3.");
        day3::solve(DAY_3_INPUT);
    }

    // Day 4
    if args.all || args.day4 {
        println!("Solving Day 4.");
        day4::solve(DAY_4_INPUT);
    }

    // Day 5
    if args.all || args.day5 {
        println!("Solving Day 5.");
        day5::solve(DAY_5_INPUT);
    }

    // Day 6
    if args.all || args.day6 {
        println!("Solving Day 6.");
        day6::solve(DAY_6_INPUT);
    }

    // Day 7
    if args.all || args.day7 {
        println!("Solving Day 7.");
        day7::solve(DAY_7_INPUT);
    }

    // Day 8
    if args.all || args.day8 {
        println!("Solving Day 8.");
        day8::solve(DAY_8_INPUT);
    }
}
