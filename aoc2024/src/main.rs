use aoc2024::days;
use clap::Parser;

macro_rules! solve_day_inner {
    ($nb:literal, $mod:ident, $args:ident, $input:ident) => {
        // Day $nb
        if $args.all || $args.days.contains(&$nb) {
            println!("Solving Day {}.", $nb);
            days::$mod::solve($input);
        }
    };
}

macro_rules! solve_day {
    ($nb:literal, $args:ident) => {
        solve_day!(
            $nb,
            concat_idents!(day, $nb),
            args,
            concat_idents!(DAY_, $nb, _INPUT)
        )
    };
    ($nb:literal, $mod:expr, $args:tt, $input:ident) => {
        assert!($nb <= 24, "days stop at 24");
        assert!($nb > 0, "days start at 1");
        solve_day_inner!($nb, $mod, $args, $input)
    };
}

const DAY_9_INPUT: &str = include_str!("input/day09.txt");
const DAY_10_INPUT: &str = include_str!("input/day10.txt");
const DAY_11_INPUT: &str = include_str!("input/day11.txt");
const DAY_13_INPUT: &str = include_str!("input/day13.txt");
const DAY_14_INPUT: &str = include_str!("input/day14.txt");
const DAY_15_INPUT: &str = include_str!("input/day15.txt");
const DAY_16_INPUT: &str = include_str!("input/day16.txt");
const DAY_17_INPUT: &str = include_str!("input/day17.txt");
const DAY_18_INPUT: &str = include_str!("input/day18.txt");
const DAY_20_INPUT: &str = include_str!("input/day20.txt");
const DAY_21_INPUT: &str = include_str!("input/day21.txt");
const DAY_22_INPUT: &str = include_str!("input/day22.txt");
const DAY_23_INPUT: &str = include_str!("input/day23.txt");
const DAY_24_INPUT: &str = include_str!("input/day24.txt");

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    all: bool,

    #[arg(long, short)]
    days: Vec<usize>,
}

fn main() {
    let args = Args::parse();

    // solve_day!(1, args);
    // Day 1
    if args.all || args.days.contains(&1) {
        println!("Solving Day {}.", 1);
        let results = days::day01::solve();
        println!("Part 1 result: {:?}", results.0);
        println!("Part 2 result: {:?}", results.1);
    }
    // Day 2
    if args.all || args.days.contains(&2) {
        println!("Solving Day {}.", 2);
        let results = days::day02::solve();
        println!("Part 1 result: {:?}", results.0);
        println!("Part 2 result: {:?}", results.1);
    }
    // Day 3
    if args.all || args.days.contains(&3) {
        println!("Solving Day {}.", 3);
        let results = days::day03::solve();
        println!("Part 1 result: {:?}", results.0);
        println!("Part 2 result: {:?}", results.1);
    }
    // Day 4
    if args.all || args.days.contains(&4) {
        println!("Solving Day {}.", 4);
        let results = days::day04::solve();
        println!("Part 1 result: {:?}", results.0);
        println!("Part 2 result: {:?}", results.1);
    }
    // Day 5
    if args.all || args.days.contains(&5) {
        println!("Solving Day {}.", 5);
        let results = days::day05::solve();
        println!("Part 1 result: {:?}", results.0);
        println!("Part 2 result: {:?}", results.1);
    }
    // Day 6
    if args.all || args.days.contains(&6) {
        println!("Solving Day {}.", 6);
        let results = days::day06::solve();
        println!("Part 1 result: {:?}", results.0);
        println!("Part 2 result: {:?}", results.1);
    }
    // Day 7
    if args.all || args.days.contains(&7) {
        println!("Solving Day {}.", 7);
        let results = days::day07::solve();
        println!("Part 1 result: {:?}", results.0);
        println!("Part 2 result: {:?}", results.1);
    }
    // Day 8
    if args.all || args.days.contains(&8) {
        println!("Solving Day {}.", 8);
        let results = days::day08::solve();
        println!("Part 1 result: {:?}", results.0);
        println!("Part 2 result: {:?}", results.1);
    }
    // Day 9
    if args.all || args.days.contains(&9) {
        println!("Solving Day {}.", 9);
        days::day09::solve(DAY_9_INPUT);
    }
    // Day 10
    if args.all || args.days.contains(&10) {
        println!("Solving Day {}.", 10);
        days::day10::solve(DAY_10_INPUT);
    }
    // Day 11
    if args.all || args.days.contains(&11) {
        println!("Solving Day {}.", 11);
        days::day11::solve(DAY_11_INPUT);
    }
    // Day 12
    if args.all || args.days.contains(&12) {
        println!("Solving Day {}.", 12);
        let results = days::day12::solve();
        println!("Part 1 result: {:?}", results.0);
        println!("Part 2 result: {:?}", results.1);
    }
    // Day 13
    if args.all || args.days.contains(&13) {
        println!("Solving Day {}.", 13);
        let results = days::day13::solve();
        println!("Part 1 result: {:?}", results.0);
        println!("Part 2 result: {:?}", results.1);
    }
    // Day 14
    if args.all || args.days.contains(&14) {
        println!("Solving Day {}.", 14);
        let results = days::day14::solve();
        println!("Part 1 result: {:?}", results.0);
        println!("Part 2 result: {:?}", results.1);
    }
    // Day 15
    if args.all || args.days.contains(&15) {
        println!("Solving Day {}.", 15);
        // let results = days::day15::solve();
        // println!("Part 1 result: {:?}", results.0);
        // println!("Part 2 result: {:?}", results.1);
    }
    // Day 16
    if args.all || args.days.contains(&16) {
        println!("Solving Day {}.", 16);
        let results = days::day16::solve();
        println!("Part 1 result: {:?}", results.0);
        println!("Part 2 result: {:?}", results.1);
    }
    // Day 17
    if args.all || args.days.contains(&17) {
        println!("Solving Day {}.", 17);
        days::day17::solve(DAY_17_INPUT);
    }
    // Day 18
    if args.all || args.days.contains(&18) {
        println!("Solving Day {}.", 18);
        days::day18::solve(DAY_18_INPUT);
    }
    // Day 19
    if args.all || args.days.contains(&19) {
        println!("Solving Day {}.", 19);
        let results = days::day19::solve();
        println!("Part 1 result: {:?}", results.0);
        println!("Part 2 result: {:?}", results.1);
    }
    // Day 20
    if args.all || args.days.contains(&20) {
        println!("Solving Day {}.", 20);
        days::day20::solve(DAY_20_INPUT);
    }
    // Day 21
    if args.all || args.days.contains(&21) {
        println!("Solving Day {}.", 21);
        days::day21::solve(DAY_21_INPUT);
    }
    // Day 22
    if args.all || args.days.contains(&22) {
        println!("Solving Day {}.", 22);
        days::day22::solve(DAY_22_INPUT);
    }
    // Day 23
    if args.all || args.days.contains(&23) {
        println!("Solving Day {}.", 23);
        days::day23::solve(DAY_23_INPUT);
    }
    // Day 24
    if args.all || args.days.contains(&24) {
        println!("Solving Day {}.", 24);
        days::day24::solve(DAY_24_INPUT);
    }
}
