use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, multispace1},
    multi::many1,
    sequence::{delimited, preceded},
    IResult,
};

use crate::{common, input::DayInput, Day};

pub struct Day6;

impl Day for Day6 {
    type Input = Competition;

    const DAY_NO: usize = 6;

    fn solve_challenge_1(input: &Self::Input) -> u32 {
        input.races.iter().map(Race::solve).product()
    }

    fn solve_challenge_2(input: &Self::Input) -> u32 {
        0
    }
}

pub struct Competition {
    races: Vec<Race>,
}

struct Race {
    time: u32,
    record: u32,
}
impl Race {
    fn new(time: u32, record: u32) -> Self {
        Self { time, record }
    }

    fn solve(&self) -> u32 {
        // x = hold time
        // d = race duration
        // r = record distance

        // (d - x) * x > r
        // (d - x) * x - r > 0
        // (d - x) * x - r = 0
        // dx - x² - r = 0
        // -x² + dx - r = 0

        // Now apply the quadratic formula:
        // (-b ± √(b² - 4ac)) / 2a
        // Substituting our variables gives:
        // (-d ± √(d² - 4r)) / -2

        let d = self.time as f32;
        let r = self.record as f32;

        let discriminant = (d.powi(2) - 4.0 * r).sqrt();

        let lower_bound = (-d + discriminant) / -2.0;
        let upper_bound = (-d - discriminant) / -2.0;

        let solutions = (lower_bound.ceil() as u32)..(upper_bound.ceil() as u32);
        solutions.len() as u32
    }
}

impl DayInput for Competition {
    fn load(input: &'static str) -> Self {
        common::parse(competition, input)
    }
}

fn competition(i: &str) -> IResult<&str, Competition> {
    let (i, times) = number_list("Time:")(i)?;
    let (i, records) = number_list("Distance:")(i)?;

    let races = times
        .into_iter()
        .zip(records)
        .map(|(t, r)| Race::new(t, r))
        .collect();

    Ok((i, Competition { races }))
}

fn number_list<'i>(label: &'static str) -> impl FnMut(&'i str) -> IResult<&'i str, Vec<u32>> {
    delimited(
        tag(label),
        many1(preceded(multispace1, common::u32)),
        line_ending,
    )
}
