use arrayvec::ArrayVec;

use crate::Day;

pub struct Day01;
impl Day for Day01 {
    type Input = Vec<&'static str>;

    const DAY_NO: usize = 1;

    fn solve_challenge_1(input: &Self::Input) -> u64 {
        input.iter().map(|&l| calibrate(l)).sum()
    }

    fn solve_challenge_2(input: &Self::Input) -> u64 {
        input.iter().map(|&l| calibrate_spelled_out(l)).sum()
    }
}

fn calibrate(line: &str) -> u64 {
    let digits = line.chars().filter_map(|c| c.to_digit(10).map(|d| d as u8));

    calculate_calibration(digits)
}

fn calibrate_spelled_out(line: &str) -> u64 {
    let replacements = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut digits: ArrayVec<u8, 16> = ArrayVec::new();

    for idx in 0..line.len() {
        let slice = &line[idx..];

        if let Some((_, rep)) = replacements.iter().find(|(n, _)| slice.starts_with(n)) {
            digits.push(*rep);
        } else if let Some(d) = slice.chars().next().and_then(|c| c.to_digit(10)) {
            digits.push(d as u8);
        }
    }

    calculate_calibration(digits.into_iter())
}

fn calculate_calibration<I: DoubleEndedIterator<Item = u8> + Clone>(mut iter: I) -> u64 {
    let first = iter.next().unwrap();
    let last = iter.next_back().unwrap_or(first) as u64;
    let first = first as u64;

    first * 10 + last
}
