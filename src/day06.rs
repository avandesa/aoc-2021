use std::collections::HashMap;

/// Store the number of days left in the spawning period when this fish was
/// created. For the top-level fish that come as part of the input, this is
/// `total_days - starting_counter`.
#[derive(Debug, Clone, Copy)]
struct Fish(usize);

impl Fish {
    /// Recursively count the size of the 'family tree' this fish will produce,
    /// and memoize the result
    fn count_children(self, memo: &mut HashMap<usize, usize>) -> usize {
        if self.0 == 0 {
            // This fish will not spawn any others, return 1 for itself
            1
        } else if let Some(memoized_count) = memo.get(&self.0) {
            *memoized_count
        } else {
            let count = (1..=self.0)
                .rev() // Count down the days remaining
                .step_by(7) // Every seven days ...
                .map(|days_left| {
                    Fish(days_left.saturating_sub(9)) // ... spawn a new fish ...
                        .count_children(memo) // ... and count how many others it will spawn
                })
                .sum::<usize>()
                + 1usize; // Add one for this fish

            // Memoize the result so we don't have to re-do the calculation
            memo.insert(self.0, count);
            count
        }
    }
}

#[derive(Debug)]
struct School(Vec<Fish>);

impl School {
    fn total(&self) -> usize {
        let mut memo = HashMap::new();
        self.0.iter().map(|f| f.count_children(&mut memo)).sum()
    }
}

pub fn part1(input: String) {
    let school = parse_input(&input, 80);
    println!("Number of fish: {}", school.total());
}

pub fn part2(input: String) {
    let school = parse_input(&input, 256);
    println!("Number of fish: {}", school.total());
}

fn parse_input(input: &str, days: usize) -> School {
    School(
        input
            .split(',')
            .map(|n| Fish(days - n.parse::<usize>().unwrap()))
            .collect(),
    )
}
