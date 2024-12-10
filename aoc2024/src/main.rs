use clap::Parser;
mod days;

macro_rules! solve_day {
    ($nb:ident) => {
        // Day $nb
        if args.all || args.days.contains(&$nb) {
            println!("Solving Day $nb.");
            days::day01::solve(DAY_1_INPUT);
        }
    };
}

const DAY_1_INPUT: &str = include_str!("../input/day01.txt");
const DAY_2_INPUT: &str = include_str!("../input/day02.txt");
const DAY_3_INPUT: &str = include_str!("../input/day03.txt");
const DAY_4_INPUT: &str = include_str!("../input/day04.txt");
const DAY_5_INPUT: &str = include_str!("../input/day05.txt");
const DAY_6_INPUT: &str = include_str!("../input/day06.txt");
const DAY_7_INPUT: &str = include_str!("../input/day07.txt");
const DAY_8_INPUT: &str = include_str!("../input/day08.txt");
const DAY_9_INPUT: &str = include_str!("../input/day09.txt");
const DAY_10_INPUT: &str = include_str!("../input/day10.txt");
const DAY_11_INPUT: &str = include_str!("../input/day11.txt");
const DAY_12_INPUT: &str = include_str!("../input/day12.txt");
const DAY_13_INPUT: &str = include_str!("../input/day13.txt");
const DAY_14_INPUT: &str = include_str!("../input/day14.txt");
const DAY_15_INPUT: &str = include_str!("../input/day15.txt");
const DAY_16_INPUT: &str = include_str!("../input/day16.txt");
const DAY_17_INPUT: &str = include_str!("../input/day17.txt");
const DAY_18_INPUT: &str = include_str!("../input/day18.txt");
const DAY_19_INPUT: &str = include_str!("../input/day19.txt");
const DAY_20_INPUT: &str = include_str!("../input/day20.txt");
const DAY_21_INPUT: &str = include_str!("../input/day21.txt");
const DAY_22_INPUT: &str = include_str!("../input/day22.txt");
const DAY_23_INPUT: &str = include_str!("../input/day23.txt");
const DAY_24_INPUT: &str = include_str!("../input/day24.txt");

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    all: bool,

    #[arg(long, short)]
    days: Vec<usize>,
}

fn main() {
    let args = Args::parse();
    println!("Days : {:?}", args.days);
    days::day01::solve("");

    //solve_day!(1);

    //     // Day 1
    //     if args.all || args.days.contains(&1) {
    //         println!("Solving Day 1.");
    //         aoc2024::days::day01::solve(DAY_1_INPUT);
    //     }
    //
    //     // Day 2
    //     if args.all || args.days.contains(&2) {
    //         println!("Solving Day 2.");
    //         day2::solve(DAY_2_INPUT);
    //     }
    //
    //     // Day 3
    //     if args.all || args.days.contains(&1) {
    //         println!("Solving Day 3.");
    //         day3::solve(DAY_3_INPUT);
    //     }
    //
    //     // Day 4
    //     if args.all || args.days.contains(&1) {
    //         println!("Solving Day 4.");
    //         day4::solve(DAY_4_INPUT);
    //     }
    //
    //     // Day 5
    //     if args.all || args.days.contains(&1) {
    //         println!("Solving Day 5.");
    //         day5::solve(DAY_5_INPUT);
    //     }
    //
    //     // Day 6
    //     if args.all || args.days.contains(&1) {
    //         println!("Solving Day 6.");
    //         day6::solve(DAY_6_INPUT);
    //     }
    //
    //     // Day 7
    //     if args.all || args.days.contains(&1) {
    //         println!("Solving Day 7.");
    //         day7::solve(DAY_7_INPUT);
    //     }
    //
    //     // Day 8
    //     if args.all || args.days.contains(&1) {
    //         println!("Solving Day 8.");
    //         day8::solve(DAY_8_INPUT);
    //     }
    //
    //     // Day 9
    //     if args.all || args.days.contains(&1) {
    //         println!("Solving Day 9.");
    //         day9::solve(DAY_9_INPUT);
    //     }
    //
    //     // Day 10
    //     if args.all || args.days.contains(&1) {
    //         println!("Solving Day 10.");
    //         day10::solve(DAY_10_INPUT);
    //     }
}
