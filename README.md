# Advent of Code

Just saving here a few scripts I made solving some problems for the advent of
code puzzles.
It's not supposed to be super optimized or super pretty 🙃

## Run 

To run the script, `cd` into the particular year, place your input file into
the input folder with the following name:  `day{:2,x}.txt`, where x in [1..24].

You can get your input data, e.g for year 2024 day 10 here: 
https://adventofcode.com/2024/day/10/input

Then run with the day you want to run in argument:

```console
cd aoc2024/
cargo run --release -- -d10
```

## Bench

It's also possible to run `cargo bench`, make sure to cd into the actual year
before running the command:

```console
cd aoc2024/
cargo bench
```
