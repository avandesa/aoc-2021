//! This is the first one I've cheated on, lol. Apparently the ideal alignment
//! for part 1 is the median of the positions, and for part 2 it's the mean,
//! rounded down. I have no idea why. The code is all my own though.

pub fn part1(input: String) {
    let positions = parse_input(&input);
    let alignment = median(&positions);
    let cost = linear_cost(&positions, alignment);
    println!("Position: {}, Cost: {}", alignment, cost);
}

pub fn part2(input: String) {
    let positions = parse_input(&input);
    let alignment = mean(&positions);
    let cost = quadratic_cost(&positions, alignment);
    println!("Position: {}, Cost: {}", alignment, cost);
}

fn median(positions: &[u32]) -> u32 {
    let mut sorted = Vec::from(positions);
    sorted.sort();
    sorted[sorted.len() / 2]
}

fn linear_cost(positions: &[u32], alignment: u32) -> u32 {
    positions
        .iter()
        .map(|p| (*p as i32 - alignment as i32).abs() as u32)
        .sum()
}

fn mean(positions: &[u32]) -> u32 {
    positions.iter().sum::<u32>() / (positions.len() as u32)
}

fn quadratic_cost(positions: &[u32], alignment: u32) -> u32 {
    positions
        .iter()
        .map(|p| ((*p as i32) - (alignment as i32)).abs() as u32)
        .map(|distance| (1..=distance).sum::<u32>())
        .sum()
}

fn parse_input(input: &str) -> Vec<u32> {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}
