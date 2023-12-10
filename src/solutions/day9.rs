use nom::{bytes::complete::tag, combinator::map, multi::separated_list1, IResult};

use crate::{common, input::DayInput, Day};

pub struct Day9;
impl Day for Day9 {
    type Input = Vec<Sequence>;

    const DAY_NO: usize = 9;

    fn solve_challenge_1(input: &Self::Input) -> u64 {
        input.iter().map(|s| s.append()).sum::<i64>() as u64
    }

    fn solve_challenge_2(input: &Self::Input) -> u64 {
        input.iter().map(|s| s.prepend()).sum::<i64>() as u64
    }
}

pub struct Sequence {
    outer: Option<Box<Sequence>>,
    numbers: Vec<i64>,
}
impl Sequence {
    pub fn new(numbers: Vec<i64>) -> Sequence {
        let outer = if numbers.iter().all(|&n| n == 0) {
            None
        } else {
            Some(Box::new(Sequence::new(
                numbers
                    .iter()
                    .zip(numbers[1..].iter())
                    .map(|(a, b)| b - a)
                    .collect(),
            )))
        };

        Self { outer, numbers }
    }

    fn prepend(&self) -> i64 {
        let subber = self.get_outer_value(|o| o.prepend());
        let first = self.numbers[0];
        first - subber
    }

    fn append(&self) -> i64 {
        let adder = self.get_outer_value(|o| o.append());
        let last = self.numbers[self.numbers.len() - 1];
        last + adder
    }

    fn get_outer_value<F>(&self, operation: F) -> i64
    where
        F: FnOnce(&Sequence) -> i64,
    {
        match &self.outer {
            Some(o) => operation(o),
            None => 0,
        }
    }
}

impl DayInput for Sequence {
    fn load(input: &'static str) -> Self {
        common::parse(sequence, input)
    }
}

fn sequence(i: &str) -> IResult<&str, Sequence> {
    map(separated_list1(tag(" "), common::i64), Sequence::new)(i)
}
