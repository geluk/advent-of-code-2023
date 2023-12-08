use std::fmt::{self, Display, Formatter};

use arrayvec::ArrayVec;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::anychar,
    combinator::{map, map_opt},
    multi::fill,
    sequence::{pair, preceded},
    IResult,
};

use crate::{common, input::DayInput, Day};

pub struct Day7;
impl Day for Day7 {
    type Input = Vec<Hand>;

    const DAY_NO: usize = 7;

    fn solve_challenge_1(input: &Self::Input) -> u32 {
        input
            .iter()
            .sorted_by_key(|h| (h.rank, h.cards))
            .enumerate()
            .map(|(idx, c)| {
                let rank = (idx + 1) as u32;
                rank * c.bid
            })
            .sum()
    }

    fn solve_challenge_2(_input: &Self::Input) -> u32 {
        0
    }
}

#[derive(Clone, Copy)]
pub struct Hand {
    cards: Cards,
    bid: u32,
    rank: u32,
}
impl Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} rank {}",
            self.cards.map(|c| c.to_string()).join(""),
            self.rank
        )
    }
}

type Cards = [Card; 5];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Num(u32),
    Jack,
    Queen,
    King,
    Ace,
}
impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let display_char = match self {
            Card::Num(10) => "T",
            Card::Num(n) => return write!(f, "{}", n),
            Card::Jack => "J",
            Card::Queen => "Q",
            Card::King => "K",
            Card::Ace => "A",
        };
        write!(f, "{display_char}")
    }
}

impl Hand {
    fn new(cards: Cards, bid: u32) -> Hand {
        Self {
            cards,
            bid,
            rank: Self::get_rank(&cards),
        }
    }

    fn get_rank(cards: &Cards) -> u32 {
        let mut groups: ArrayVec<usize, 5> = cards
            .iter()
            .sorted()
            .group_by(|&&c| c)
            .into_iter()
            .map(|(_, items)| items.count())
            .collect();

        // Note the ordering, sort by group count descending.
        groups.sort_by(|a, b| b.cmp(a));

        match &groups[..] {
            [5] => 6,        // Five of a kind
            [4, ..] => 5,    // Four of a kind
            [3, 2] => 4,     // Full house
            [3, ..] => 3,    // Three of a kind
            [2, 2, ..] => 2, // Two pair
            [2, ..] => 1,    // One pair
            _ => 0,          // High card
        }
    }
}

impl DayInput for Hand {
    fn load(input: &'static str) -> Self {
        common::parse(hand, input)
    }
}

fn hand(i: &str) -> IResult<&str, Hand> {
    map(
        pair(cards, preceded(tag(" "), common::u32)),
        |(cards, bid)| Hand::new(cards, bid),
    )(i)
}

fn cards(i: &str) -> IResult<&str, Cards> {
    let mut cards = [Card::Num(0); 5];
    let (i, ()) = fill(card, &mut cards)(i)?;
    Ok((i, cards))
}

fn card(i: &str) -> IResult<&str, Card> {
    map_opt(anychar, |c| match c {
        'A' => Some(Card::Ace),
        'K' => Some(Card::King),
        'Q' => Some(Card::Queen),
        'J' => Some(Card::Jack),
        'T' => Some(Card::Num(10)),
        '0'..='9' => Some(Card::Num(c.to_digit(10)?)),
        _ => None,
    })(i)
}
