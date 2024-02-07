pub fn solve(input: &str) -> (u32, u32) {
    let mut totals: Vec<u32> = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(str::parse::<u32>)
                .filter_map(Result::ok)
                .sum()
        })
        .collect();

    totals.sort();

    (part_1(&totals), part_2(&totals))
}

fn part_1(totals: &[u32]) -> u32 {
    totals.iter().max().copied().unwrap_or(0)
}

fn part_2(totals: &[u32]) -> u32 {
    totals.iter().rev().take(3).sum()
}
