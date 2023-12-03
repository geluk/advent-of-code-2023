use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{eof, map_res},
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated},
};

use crate::{input::DayInput, Day};

pub struct Day2;

impl Day for Day2 {
    type Input = Vec<Game>;

    const DAY_NO: usize = 2;

    fn solve_challenge_1(input: &Self::Input) -> u32 {
        let max_draw = Draw {
            red: 12,
            green: 13,
            blue: 14,
        };

        let sum = input
            .iter()
            .filter(|g| g.is_possible(&max_draw))
            .map(|g| g.game_no)
            .sum();

        sum
    }

    fn solve_challenge_2(input: &Self::Input) -> u32 {
        input.iter().map(|g| g.required_draw().power()).sum()
    }
}

pub struct Game {
    game_no: u32,
    draws: Vec<Draw>,
}
impl Game {
    fn is_possible(&self, max_draw: &Draw) -> bool {
        self.draws.iter().all(|d| d.can_draw_from(max_draw))
    }

    fn required_draw(&self) -> Draw {
        self.draws
            .iter()
            .copied()
            .reduce(|acc, n| acc.required_draw(&n))
            .unwrap()
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}
impl Draw {
    fn can_draw_from(&self, parent: &Draw) -> bool {
        self.red <= parent.red && self.green <= parent.green && self.blue <= parent.blue
    }

    fn required_draw(&self, other: &Draw) -> Draw {
        Draw {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

impl DayInput for Game {
    fn load(input: &'static str) -> Self {
        parse(input).unwrap()
    }
}

fn parse(i: &'static str) -> Result<Game> {
    let (i, game_no) = delimited(tag("Game "), number, tag(": "))(i)?;
    let (_, draws) = terminated(separated_list1(tag("; "), draw), eof)(i)?;

    Ok(Game { game_no, draws })
}

fn draw(i: &str) -> nom::IResult<&str, Draw> {
    let (i, colors) = separated_list1(tag(", "), color)(i)?;

    let mut draw: Draw = Default::default();

    for (num, color) in colors {
        match color {
            "red" => {
                draw.red = num;
            }
            "green" => {
                draw.green = num;
            }
            "blue" => {
                draw.blue = num;
            }
            _ => unreachable!(),
        }
    }

    Ok((i, draw))
}

fn color(i: &str) -> nom::IResult<&str, (u32, &str)> {
    separated_pair(
        number,
        tag(" "),
        alt((tag("red"), tag("green"), tag("blue"))),
    )(i)
}

fn number(i: &str) -> nom::IResult<&str, u32> {
    map_res(digit1, |r: &str| r.parse())(i)
}
