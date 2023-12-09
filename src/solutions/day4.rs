use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    multi::{fold_many1, many1},
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

use crate::{common, input::DayInput, Day};

pub struct Day4;
impl Day for Day4 {
    type Input = Vec<Card>;

    const DAY_NO: usize = 4;

    fn solve_challenge_1(input: &Self::Input) -> u64 {
        Collection::new(input).score()
    }

    fn solve_challenge_2(input: &Self::Input) -> u64 {
        let mut collection = Collection::new(input);
        collection.redeem_all();
        collection.copies.iter().sum()
    }
}

pub struct Collection<'c> {
    cards: &'c [Card],
    copies: Vec<u64>,
}
impl<'c> Collection<'c> {
    fn new(cards: &'c [Card]) -> Self {
        Self {
            cards,
            copies: vec![1; cards.len()],
        }
    }

    fn score(&self) -> u64 {
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

    fn copies_of(&self, card: &Card) -> u64 {
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

    pub fn score(&self) -> u64 {
        1 << self.win_count >> 1
    }
}

impl DayInput for Card {
    fn load(input: &'static str) -> Self {
        common::parse(card, input)
    }
}

fn card(i: &str) -> IResult<&str, Card> {
    let (i, card_no) = delimited(tag("Card"), whitespace_number, tag(":"))(i)?;
    let (i, (winning_numbers, have_numbers)) =
        separated_pair(number_set, tag(" |"), number_set)(i)?;

    let card = Card::new(card_no as usize, winning_numbers, have_numbers);
    Ok((i, card))
}

fn number_set(i: &str) -> IResult<&str, HashSet<u32>> {
    let add_number = |mut set: HashSet<u32>, num| {
        set.insert(num);
        set
    };

    fold_many1(whitespace_number, HashSet::new, add_number)(i)
}

fn whitespace_number(i: &str) -> IResult<&str, u32> {
    preceded(many1(tag(" ")), common::u32)(i)
}
