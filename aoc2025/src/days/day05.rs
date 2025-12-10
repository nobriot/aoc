use crate::input;

pub fn solve() -> (Option<usize>, Option<usize>) {
    let input = input::DAY_5_INPUT;

    let mut fresh_ingredients = Vec::new();
    let mut ingredients = Vec::new();

    // Parse the input
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line.contains('-') {
            // Parse a range
            let (part1, part2) = line.split_once('-').unwrap();
            let part1 = part1.parse::<usize>().expect("Valid input file");
            let part2 = part2.parse::<usize>().expect("Valid input file");

            fresh_ingredients.push((part1, part2));
        } else {
            // Parse an ingredient
            let ingredient: usize = line.parse().expect("Valid input file");
            ingredients.push(ingredient);
        }
    }

    // Now optimize a bit and sort, then collapse the overlapping ranges.
    fresh_ingredients.sort_unstable_by_key(|(start, _)| *start);
    let mut optimized = Vec::with_capacity(fresh_ingredients.len());

    let mut current = fresh_ingredients[0];
    for &(start, stop) in &fresh_ingredients[1..] {
        if start <= current.1 + 1 {
            // The start of the next block is before, or one step after the end of the previous
            current = (start.min(current.0), stop.max(current.1));
        } else {
            optimized.push(current);
            current = (start, stop);
        }
    }
    optimized.push(current);

    // Sanity checking
    // let mut iter = optimized.iter().peekable();
    // while let Some(&(s, e)) = iter.next() {
    //     if let Some(&n) = iter.peek() {
    //         assert!(n.0 > s + 1);
    //         assert!(n.0 > e + 1);
    //         assert!(n.1 > e + 1);
    //     }
    // }

    // Now count the fresh (part 1)
    let mut fresh_count = 0;
    for &ingredient in &ingredients {
        // Binary search would be better here.
        for &safe in &optimized {
            if ingredient >= safe.0 && ingredient <= safe.1 {
                fresh_count += 1;
                break;
            } else if ingredient < safe.0 {
                break;
            }
        }
    }

    // Count how many individual ingredients coult be considered fresh
    let mut potential_fresh_ingredients = 0;
    for &range in &optimized {
        potential_fresh_ingredients += range.1 - range.0 + 1;
    }

    (Some(fresh_count), Some(potential_fresh_ingredients))
}
