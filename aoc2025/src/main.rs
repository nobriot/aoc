use aoc2025::days;
use aoc_macros::solve_days;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    all: bool,

    #[arg(long, short)]
    days: Vec<usize>,
}

fn main() {
    let args = Args::parse();

    // Solve the requested days in argument
    solve_days!(1, 8, args);
}
