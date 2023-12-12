use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, multispace1},
    multi::{fold_many1, many1},
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};
use rustc_hash::FxHashSet;

use crate::{common, input::DayInput, Day};

pub struct Day04;
impl Day for Day04 {
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
    pub fn new(
        card_no: usize,
        winning_numbers: &FxHashSet<u32>,
        have_numbers: &FxHashSet<u32>,
    ) -> Self {
        let win_count = winning_numbers.intersection(have_numbers).count();

        Self {
            index: card_no - 1,
            win_count,
        }
    }

    pub fn score(&self) -> u64 {
        1 << self.win_count >> 1
    }
}

impl DayInput for Vec<Card> {
    fn load(input: &'static str) -> Self {
        common::parse(cards, input)
    }
}

fn cards(i: &str) -> IResult<&str, Vec<Card>> {
    let mut winning_buffer = FxHashSet::with_capacity_and_hasher(10, Default::default());
    let mut have_buffer = FxHashSet::with_capacity_and_hasher(25, Default::default());

    many1(move |i| card(i, &mut winning_buffer, &mut have_buffer))(i)
}

fn card<'i, 'b>(
    i: &'i str,
    winning: &'b mut FxHashSet<u32>,
    have: &'b mut FxHashSet<u32>,
) -> IResult<&'i str, Card> {
    let (i, card_no) = delimited(tag("Card"), whitespace_number, tag(":"))(i)?;
    let (i, _) = terminated(
        separated_pair(
            |i| number_set(i, winning),
            tag(" |"),
            |i| number_set(i, have),
        ),
        line_ending,
    )(i)?;

    let card = Card::new(card_no as usize, winning, have);
    Ok((i, card))
}

fn number_set<'i>(i: &'i str, number_buffer: &mut FxHashSet<u32>) -> IResult<&'i str, ()> {
    number_buffer.clear();

    fold_many1(
        whitespace_number,
        || (),
        |(), n| {
            number_buffer.insert(n);
        },
    )(i)
}

fn whitespace_number(i: &str) -> IResult<&str, u32> {
    preceded(multispace1, common::u32)(i)
}
