use std::cmp::Ordering;

use arrayvec::ArrayVec;

const LINE_LENGTH: usize = 12;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bit {
    Zero,
    One,
}

impl Bit {
    fn from_char(c: char) -> Self {
        match c {
            '0' => Bit::Zero,
            '1' => Bit::One,
            _ => panic!("Invalid bit character: {}", c),
        }
    }

    fn invert(&self) -> Self {
        match self {
            Bit::Zero => Bit::One,
            Bit::One => Bit::Zero,
        }
    }

    fn as_u32(&self) -> u32 {
        match self {
            Bit::Zero => 0,
            Bit::One => 1,
        }
    }
}

trait Number {
    fn as_u32(&self) -> u32;
    fn invert(&self) -> Self;
    fn as_string(&self) -> String;
}

impl Number for ArrayVec<Bit, LINE_LENGTH> {
    fn as_u32(&self) -> u32 {
        self.iter()
            .rev()
            .enumerate()
            .fold(0, |num, (i, b)| num + b.as_u32() * 2u32.pow(i as u32))
    }

    fn invert(&self) -> Self {
        self.iter().map(|b| b.invert()).collect()
    }

    fn as_string(&self) -> String {
        self.iter()
            .map(|b| match b {
                Bit::Zero => '0',
                Bit::One => '1',
            })
            .collect()
    }
}

pub fn part1(input: String) {
    let cols = parse_cols(&input);

    let gamma = cols
        .iter()
        .map(|col| most_common_bit(col).unwrap_or(Bit::Zero))
        .collect::<ArrayVec<_, LINE_LENGTH>>();
    let episilon = gamma.invert();

    let result = gamma.as_u32() * episilon.as_u32();
    println!(
        "Result: gamma * epsilon = {} * {} = {}",
        gamma.as_u32(),
        episilon.as_u32(),
        result
    );
}

pub fn part2(input: String) {
    let numbers = parse_numbers(&input);

    let oxygen = find_oxygen_rating(numbers.clone());
    let co2 = find_co2_rating(numbers.clone());
    let result = co2.as_u32() * oxygen.as_u32();

    println!(
        "Result: oxygen * co2 = {} * {} = {}",
        oxygen.as_u32(),
        co2.as_u32(),
        result
    );
}

fn parse_numbers(input: &str) -> Vec<ArrayVec<Bit, LINE_LENGTH>> {
    input
        .lines()
        .map(|s| s.chars().take(LINE_LENGTH).map(Bit::from_char).collect())
        .collect()
}

fn cols_from_blob(bits: &[Bit]) -> [Vec<Bit>; LINE_LENGTH] {
    let mut cols: [Vec<Bit>; LINE_LENGTH] = Default::default();
    for (col, bit) in bits
        .into_iter()
        .enumerate()
        .map(|(i, b)| (i % LINE_LENGTH, b))
    {
        cols[col].push(*bit);
    }

    cols
}

fn parse_cols(input: &str) -> [Vec<Bit>; LINE_LENGTH] {
    let bits: Vec<Bit> = input
        .lines()
        .flat_map(|s| {
            s.chars()
                .take(LINE_LENGTH)
                .map(Bit::from_char)
                .collect::<ArrayVec<_, LINE_LENGTH>>() // TODO: fill a slice?
        })
        .collect();

    cols_from_blob(&bits)
}

fn cols_from_numbers(numbers: &[ArrayVec<Bit, LINE_LENGTH>]) -> [Vec<Bit>; LINE_LENGTH] {
    let bits: Vec<Bit> = numbers.iter().flatten().copied().collect();
    cols_from_blob(&bits)
}

fn most_common_bit(column: &[Bit]) -> Option<Bit> {
    let num_zeroes = column.iter().filter(|b| **b == Bit::Zero).count();
    match num_zeroes.cmp(&(column.len() / 2)) {
        Ordering::Less => Some(Bit::One),
        Ordering::Equal => None,
        Ordering::Greater => Some(Bit::Zero),
    }
}

fn least_common_bit(column: &[Bit]) -> Option<Bit> {
    most_common_bit(column).map(|b| b.invert())
}

fn check_oxygen(num: &[Bit], pos: usize, most_common: Option<Bit>) -> bool {
    debug_assert!(pos < num.len());
    match most_common {
        Some(most_common) => num[pos] == most_common,
        None => num[pos] == Bit::One,
    }
}

fn find_oxygen_rating(numbers: Vec<ArrayVec<Bit, LINE_LENGTH>>) -> ArrayVec<Bit, LINE_LENGTH> {
    let mut pos = 0;
    let mut result_set = numbers.clone();
    while result_set.len() > 1 && pos < LINE_LENGTH {
        let cols = cols_from_numbers(&result_set);
        let most_common = most_common_bit(&cols[pos]);
        result_set = result_set
            .into_iter()
            .filter(|num| check_oxygen(num.as_slice(), pos, most_common))
            .collect();
        pos += 1;
    }

    result_set[0].clone()
}

fn check_co2(num: &[Bit], pos: usize, least_common: Option<Bit>) -> bool {
    debug_assert!(pos < num.len());
    match least_common {
        Some(least_common) => num[pos] == least_common,
        None => num[pos] == Bit::Zero,
    }
}

fn find_co2_rating(numbers: Vec<ArrayVec<Bit, LINE_LENGTH>>) -> ArrayVec<Bit, LINE_LENGTH> {
    let mut pos = 0;
    let mut result_set = numbers.clone();
    while result_set.len() > 1 && pos < LINE_LENGTH {
        let cols = cols_from_numbers(&result_set);
        let least_common = least_common_bit(&cols[pos]);
        result_set = result_set
            .into_iter()
            .filter(|num| check_co2(num.as_slice(), pos, least_common))
            .collect();
        pos += 1;
    }

    result_set[0].clone()
}
