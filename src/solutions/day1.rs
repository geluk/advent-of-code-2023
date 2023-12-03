use crate::Day;

pub struct Day1;

impl Day for Day1 {
    type Input = Vec<&'static str>;

    const DAY_NO: usize = 1;

    fn solve_challenge_1(input: &Self::Input) -> u32 {
        input.iter().map(|&l| calculate_value(l)).sum()
    }

    fn solve_challenge_2(input: &Self::Input) -> u32 {
        input.iter().map(|&l| calculate_value_advanced(l)).sum()
    }
}

fn calculate_value(line: &str) -> u32 {
    let mut character_iterator = line.chars().flat_map(|c| c.to_digit(10));

    let first = character_iterator.clone().next().unwrap();
    let last = character_iterator.next_back().unwrap();

    first * 10 + last
}

fn calculate_value_advanced(line: &str) -> u32 {
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

    let mut values = Vec::new();

    for idx in 0..line.len() {
        let slice = &line[idx..];

        if let Some((_, r)) = replacements.iter().find(|(n, _)| slice.starts_with(n)) {
            values.push(*r);
        } else if let Some(x) = slice.chars().next().and_then(|c| c.to_digit(10)) {
            values.push(x);
        }
    }

    let first = values.first().unwrap();
    let last = values.last().unwrap();

    first * 10 + last
}
