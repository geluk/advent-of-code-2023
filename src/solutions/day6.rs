use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, multispace1},
    multi::many1,
    sequence::{delimited, preceded},
    IResult,
};

use crate::{common, input::DayInput, Day};

pub struct Day6;
impl Day for Day6 {
    type Input = Competition;

    const DAY_NO: usize = 6;

    fn solve_challenge_1(input: &Self::Input) -> u64 {
        input.races_incorrect.iter().map(Race::solve).product()
    }

    fn solve_challenge_2(input: &Self::Input) -> u64 {
        input.race.solve()
    }
}

pub struct Competition {
    // The competition, interpreted incorrectly, representing multiple races:
    races_incorrect: Vec<Race>,
    // The competition, interpreted correctly, with just the one race:
    race: Race,
}

struct Race {
    duration: u64,
    record: u64,
}
impl Race {
    fn new(time: u64, record: u64) -> Self {
        Self {
            duration: time,
            record,
        }
    }

    fn solve(&self) -> u64 {
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

        let d = self.duration as f64;
        let r = self.record as f64;

        let discriminant = (d.powi(2) - 4.0 * r).sqrt();

        let lower_bound = (-d + discriminant) / -2.0;
        let upper_bound = (-d - discriminant) / -2.0;

        let solution_count = upper_bound.ceil() - lower_bound.ceil();
        solution_count as u64
    }
}

impl DayInput for Competition {
    fn load(input: &'static str) -> Self {
        common::parse(competition, input)
    }
}

fn competition(i: &str) -> IResult<&str, Competition> {
    let (_, races) = races(i)?;
    let (i, race) = race(i)?;

    Ok((
        i,
        Competition {
            race,
            races_incorrect: races,
        },
    ))
}

fn races(i: &str) -> IResult<&str, Vec<Race>> {
    let (i, times) = number_list("Time:")(i)?;
    let (i, records) = number_list("Distance:")(i)?;

    let races = times
        .into_iter()
        .zip(records)
        .map(|(t, r)| Race::new(t, r))
        .collect();

    Ok((i, races))
}

fn race(i: &str) -> IResult<&str, Race> {
    let (i, time) = spaced_number("Time:")(i)?;
    let (i, record) = spaced_number("Distance:")(i)?;

    Ok((i, Race::new(time, record)))
}

fn number_list<'i>(label: &'static str) -> impl FnMut(&'i str) -> IResult<&'i str, Vec<u64>> {
    delimited(
        tag(label),
        many1(preceded(multispace1, common::u64)),
        line_ending,
    )
}

fn spaced_number<'i>(label: &'static str) -> impl FnMut(&'i str) -> IResult<&'i str, u64> {
    move |i: &'i str| {
        let (i, digits) = delimited(
            tag(label),
            many1(preceded(multispace1, digit1)),
            line_ending,
        )(i)?;

        let num = digits.join("").parse().unwrap();
        Ok((i, num))
    }
}
