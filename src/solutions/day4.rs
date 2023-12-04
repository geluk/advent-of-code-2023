use std::collections::HashSet;

use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{eof, map_res},
    multi::{fold_many1, many1},
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};

use crate::{input::DayInput, Day};

pub struct Day4;

impl Day for Day4 {
    type Input = Vec<Card>;

    const DAY_NO: usize = 4;

    fn solve_challenge_1(input: &Self::Input) -> u32 {
        Collection::new(input).score() as u32
    }

    fn solve_challenge_2(input: &Self::Input) -> u32 {
        let mut collection = Collection::new(input);
        collection.redeem_all();
        collection.copies.iter().sum::<usize>() as u32
    }
}

pub struct Collection<'c> {
    cards: &'c [Card],
    copies: Vec<usize>,
}
impl<'c> Collection<'c> {
    fn new(cards: &'c [Card]) -> Self {
        Self {
            cards,
            copies: vec![1; cards.len()],
        }
    }

    fn score(&self) -> usize {
        self.cards
            .iter()
            .map(|c| c.score() * self.copies_of(c))
            .sum()
    }

    fn redeem_all(&mut self) {
        for card in self.cards.iter() {
            self.redeem(card);
        }
    }

    fn redeem(&mut self, card: &Card) {
        let next_index = card.index + 1;

        for sub_copy_idx in next_index..(next_index + card.win_count) {
            self.copies[sub_copy_idx] += self.copies_of(card);
        }
    }

    fn copies_of(&self, card: &Card) -> usize {
        self.copies[card.index]
    }
}

pub struct Card {
    index: usize,
    win_count: usize,
}
impl Card {
    pub fn new(card_no: usize, winning_numbers: HashSet<u32>, have_numbers: HashSet<u32>) -> Self {
        let win_count = winning_numbers.intersection(&have_numbers).count();

        Self {
            index: card_no - 1,
            win_count,
        }
    }

    pub fn score(&self) -> usize {
        1 << self.win_count >> 1
    }
}

impl DayInput for Card {
    fn load(input: &'static str) -> Self {
        fn load_internal(i: &'static str) -> Result<Card> {
            let (i, card_no) = delimited(tag("Card"), whitespace_number, tag(":"))(i)?;
            let (_, (winning_numbers, have_numbers)) =
                terminated(separated_pair(number_set, tag(" |"), number_set), eof)(i)?;

            Ok(Card::new(card_no as usize, winning_numbers, have_numbers))
        }

        load_internal(input).unwrap()
    }
}

fn number_set(input: &str) -> IResult<&str, HashSet<u32>> {
    let add_number = |mut set: HashSet<u32>, num| {
        set.insert(num);
        set
    };

    fold_many1(whitespace_number, HashSet::new, add_number)(input)
}

fn whitespace_number(input: &str) -> IResult<&str, u32> {
    preceded(many1(tag(" ")), number)(input)
}

fn number(i: &str) -> nom::IResult<&str, u32> {
    map_res(digit1, |r: &str| r.parse())(i)
}
