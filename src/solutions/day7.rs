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
    type Input = Vec<Hand<ClassicCard>>;

    const DAY_NO: usize = 7;

    fn solve_challenge_1(input: &Self::Input) -> u64 {
        input
            .iter()
            .sorted_by_key(|h| (h.rank, h.cards))
            .copied()
            .enumerate()
            .map(|s| s.score())
            .sum()
    }

    fn solve_challenge_2(input: &Self::Input) -> u64 {
        input
            .iter()
            .map(|h| (*h, h.as_wild_card().promote()))
            .sorted_by_key(|(orig, promoted)| (promoted.rank, orig.cards))
            .enumerate()
            .map(|(idx, (hand, _))| (idx, hand).score())
            .sum()
    }
}

trait Score {
    fn score(&self) -> u64;
}
impl<T> Score for (usize, Hand<T>) {
    fn score(&self) -> u64 {
        let (idx, card) = self;
        let rank = (idx + 1) as u64;
        rank * card.bid
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Hand<C> {
    cards: Cards<C>,
    bid: u64,
    rank: u64,
}
impl Hand<ClassicCard> {
    fn as_wild_card(&self) -> Hand<WildCard> {
        Hand {
            cards: self.cards.map(|c| c.as_wild_card()),
            bid: self.bid,
            rank: self.rank,
        }
    }
}
impl Hand<WildCard> {
    fn promote(&self) -> Hand<WildCard> {
        fn promote_jokers(cards: Cards<WildCard>) -> Cards<WildCard> {
            let idx = cards.iter().find_position(|&&c| c == WildCard::Joker);
            match idx {
                Some((idx, _)) => {
                    if let Some((_, target_card)) = Hand::group_cards(cards.iter().copied())
                        .find(|(_, c)| *c != WildCard::Joker)
                    {
                        let mut new_cards = cards;
                        new_cards[idx] = target_card;

                        promote_jokers(new_cards)
                    } else {
                        // This can only happen if the hand is full of jokers.
                        // Promote them all to aces.
                        [WildCard::Ace; 5]
                    }
                }
                None => cards,
            }
        }

        let new_cards = promote_jokers(self.cards);
        let new_hand = Self::new(new_cards, self.bid);

        if new_hand == *self {
            *self
        } else {
            new_hand
        }
    }
}
impl<C: Eq + Ord + Copy> Hand<C> {
    fn new(cards: Cards<C>, bid: u64) -> Hand<C> {
        Self {
            cards,
            bid,
            rank: Self::get_rank(&cards),
        }
    }

    fn get_rank(cards: &Cards<C>) -> u64 {
        let group_sizes: ArrayVec<_, 5> = Self::group_cards(cards.iter().copied())
            .map(|(count, _)| count)
            .collect();

        match &group_sizes[..] {
            [5] => 6,        // Five of a kind
            [4, ..] => 5,    // Four of a kind
            [3, 2] => 4,     // Full house
            [3, ..] => 3,    // Three of a kind
            [2, 2, ..] => 2, // Two pair
            [2, ..] => 1,    // One pair
            _ => 0,          // High card
        }
    }

    fn group_cards<I: Iterator<Item = C>>(cards: I) -> impl Iterator<Item = (usize, C)> {
        cards
            .sorted()
            .group_by(|&c| c)
            .into_iter()
            .map(|(c, items)| (items.count(), c))
            // Note the ordering, sort by group count descending.
            .sorted_by(|a, b| b.cmp(a))
    }
}

type Cards<C> = [C; 5];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ClassicCard {
    Num(u8),
    Jack,
    Queen,
    King,
    Ace,
}
impl ClassicCard {
    fn as_wild_card(&self) -> WildCard {
        match self {
            ClassicCard::Num(n) => WildCard::Num(*n),
            ClassicCard::Jack => WildCard::Joker,
            ClassicCard::Queen => WildCard::Queen,
            ClassicCard::King => WildCard::King,
            ClassicCard::Ace => WildCard::Ace,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum WildCard {
    Joker,
    Num(u8),
    Queen,
    King,
    Ace,
}

impl DayInput for Hand<ClassicCard> {
    fn load(input: &'static str) -> Self {
        common::parse(hand, input)
    }
}

fn hand(i: &str) -> IResult<&str, Hand<ClassicCard>> {
    map(
        pair(cards, preceded(tag(" "), common::u64)),
        |(cards, bid)| Hand::new(cards, bid),
    )(i)
}

fn cards(i: &str) -> IResult<&str, Cards<ClassicCard>> {
    let mut cards = [ClassicCard::Num(0); 5];
    let (i, ()) = fill(card, &mut cards)(i)?;
    Ok((i, cards))
}

fn card(i: &str) -> IResult<&str, ClassicCard> {
    map_opt(anychar, |c| match c {
        'A' => Some(ClassicCard::Ace),
        'K' => Some(ClassicCard::King),
        'Q' => Some(ClassicCard::Queen),
        'J' => Some(ClassicCard::Jack),
        'T' => Some(ClassicCard::Num(10)),
        '0'..='9' => Some(ClassicCard::Num(c.to_digit(10)? as u8)),
        _ => None,
    })(i)
}
