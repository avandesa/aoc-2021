pub fn part1(input: String) {
    println!("{}", count_increments(&parse(&input)));
}

pub fn part2(input: String) {
    let windows: Vec<_> = parse(&input)
        .windows(3)
        .map(|slice| slice.iter().sum())
        .collect();
    println!("{}", count_increments(&windows));
}

fn parse(input: &str) -> Vec<u32> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

fn count_increments(nums: &[u32]) -> usize {
    nums.windows(2).filter(|slice| slice[0] < slice[1]).count()
}
