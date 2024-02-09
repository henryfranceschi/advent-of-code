use std::path::PathBuf;

use advent_of_code::{day_01, day_02};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} [day]", env!("CARGO_BIN_NAME"));
        return;
    }

    // Parse day argument.
    let Ok(day @ 1..=25): Result<u32, _> = args[1].parse() else {
        eprintln!("day must be an integer in the range [1, 25]");
        return;
    };

    // Read puzzle input file.
    let file_name = format!("day-{:0>2}", day);
    let path: PathBuf = ["input", &file_name].iter().collect();
    let Ok(input): Result<_, _> = std::fs::read_to_string(&path) else {
        eprintln!("failed to open file {:?}", &path);
        return;
    };

    let (part_1, part_2) = match day {
        1 => day_01::solve(&input),
        2 => day_02::solve(&input),
        3..=25 => todo!(),
        _ => unreachable!(),
    };

    println!("part 1: {part_1}");
    println!("part 2: {part_2}");
}
