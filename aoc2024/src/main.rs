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
        let results = days::day09::solve();
        println!("Part 1 result: {:?}", results.0);
        println!("Part 2 result: {:?}", results.1);
    }
    // Day 10
    if args.all || args.days.contains(&10) {
        println!("Solving Day {}.", 10);
        let results = days::day10::solve();
        println!("Part 1 result: {:?}", results.0);
        println!("Part 2 result: {:?}", results.1);
    }
    // Day 11
    if args.all || args.days.contains(&11) {
        println!("Solving Day {}.", 11);
        let results = days::day11::solve();
        println!("Part 1 result: {:?}", results.0);
        println!("Part 2 result: {:?}", results.1);
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
        let results = days::day15::solve();
        println!("Part 1 result: {:?}", results.0);
        println!("Part 2 result: {:?}", results.1);
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
        let results = days::day17::solve();
        println!("Part 1 result: {:?}", results.0);
        println!("Part 2 result: {:?}", results.1);
    }
    // Day 18
    if args.all || args.days.contains(&18) {
        println!("Solving Day {}.", 18);
        let results = days::day18::solve();
        println!("Part 1 result: {:?}", results.0);
        println!("Part 2 result: {:?}", results.1);
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
        let results = days::day20::solve();
        println!("Part 1 result: {:?}", results.0);
        println!("Part 2 result: {:?}", results.1);
    }
    // Day 21
    if args.all || args.days.contains(&21) {
        println!("Solving Day {}.", 21);
        let results = days::day21::solve();
        println!("Part 1 result: {:?}", results.0);
        println!("Part 2 result: {:?}", results.1);
    }
    // Day 22
    if args.all || args.days.contains(&22) {
        println!("Solving Day {}.", 22);
        let results = days::day22::solve();
        println!("Part 1 result: {:?}", results.0);
        println!("Part 2 result: {:?}", results.1);
    }
    // Day 23
    if args.all || args.days.contains(&23) {
        println!("Solving Day {}.", 23);
        let results = days::day23::solve();
        println!("Part 1 result: {:?}", results.0);
        println!("Part 2 result: {:?}", results.1);
    }
    // Day 24
    if args.all || args.days.contains(&24) {
        println!("Solving Day {}.", 24);
        let results = days::day24::solve();
        println!("Part 1 result: {:?}", results.0);
        println!("Part 2 result: {:?}", results.1);
    }
}
