use std::path::PathBuf;

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

    match day {
        1..=25 => todo!(),
        _ => unreachable!(),
    }
}
