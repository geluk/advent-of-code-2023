use std::collections::HashMap;

use itertools::Itertools;

use crate::{input::DayInput, Day};

pub struct Day3;
impl Day for Day3 {
    type Input = Schematic;

    const DAY_NO: usize = 3;

    fn solve_challenge_1(input: &Self::Input) -> u32 {
        input
            .find_numbers()
            .into_iter()
            .filter(|n| n.is_adjacent_to_symbol(input))
            .map(|i| i.value)
            .sum()
    }

    fn solve_challenge_2(input: &Self::Input) -> u32 {
        let numbers = input.find_numbers();
        let number_lookup = build_number_lookup(&numbers);

        input
            .find_gears()
            .map(|gear| input.numbers_around(gear, &number_lookup))
            .filter(|nums| nums.len() == 2)
            .map(|nums| nums.iter().map(|n| n.value).product::<u32>())
            .sum()
    }
}

fn build_number_lookup(numbers: &[Number]) -> HashMap<Point, &Number> {
    numbers
        .iter()
        .flat_map(|n| n.area().walk_lr_tb().map(move |p| (p, n)))
        .collect()
}

pub struct Schematic {
    rows: Vec<Vec<Character>>,
}
impl Schematic {
    fn find_numbers(&self) -> Vec<Number> {
        self.rows
            .iter()
            .enumerate()
            .flat_map(|(y, row)| NumbersBuilder::build(row, y))
            .collect()
    }

    fn find_gears(&self) -> impl Iterator<Item = Point> + '_ {
        self.enumerate_points()
            .filter(|p| self.lookup(*p).is_gear())
    }

    fn enumerate_points(&self) -> impl Iterator<Item = Point> {
        Area::new(Point::new(0, 0), self.right_bottom()).walk_lr_tb()
    }

    fn numbers_around<'n>(
        &self,
        point: Point,
        lookup: &HashMap<Point, &'n Number>,
    ) -> Vec<&'n Number> {
        point
            .area()
            .expand_one(self.right_bottom())
            .walk_lr_tb()
            .flat_map(|p| lookup.get(&p))
            .unique_by(|n| n.origin)
            .copied()
            .collect()
    }

    fn lookup(&self, point: Point) -> &Character {
        &self.rows[point.y][point.x]
    }

    fn width(&self) -> usize {
        self.rows[0].len()
    }

    fn height(&self) -> usize {
        self.rows.len()
    }

    fn right_bottom(&self) -> Point {
        Point::new(self.width() - 1, self.height() - 1)
    }
}

struct NumbersBuilder<'r> {
    row: &'r [Character],
    y: usize,
    numbers: Vec<Number>,
    current: Option<Number>,
}
impl<'r> NumbersBuilder<'r> {
    fn build(row: &'r [Character], y: usize) -> Vec<Number> {
        Self {
            row,
            y,
            numbers: vec![],
            current: None,
        }
        .build_numbers()
    }

    fn build_numbers(mut self) -> Vec<Number> {
        for (x, item) in self.row.iter().enumerate() {
            match item {
                Character::Digit(d) => self.get_or_create_number(x).append_digit(*d),
                _ => self.finish_number(),
            }
        }

        self.finish_number();
        self.numbers
    }

    fn get_or_create_number(&mut self, x: usize) -> &mut Number {
        self.current.get_or_insert(Number {
            origin: Point::new(x, self.y),
            length: 0,
            value: 0,
        })
    }

    fn finish_number(&mut self) {
        self.numbers.extend(self.current.take());
    }
}

#[derive(Clone, Copy)]
struct Number {
    origin: Point,
    length: usize,
    value: u32,
}
impl Number {
    fn append_digit(&mut self, digit: u32) {
        self.length += 1;
        self.value = self.value * 10 + digit;
    }

    fn is_adjacent_to_symbol(&self, schematic: &Schematic) -> bool {
        self.area()
            .expand_one(schematic.right_bottom())
            .walk_lr_tb()
            .map(|p| schematic.lookup(p))
            .any(|n| n.is_symbol())
    }

    fn area(&self) -> Area {
        Area::new(
            self.origin,
            Point::new(self.origin.x + self.length - 1, self.origin.y),
        )
    }
}

#[derive(Clone, Copy)]
enum Character {
    Empty,
    Symbol(char),
    Digit(u32),
}
impl Character {
    fn is_symbol(&self) -> bool {
        matches!(self, Character::Symbol(_))
    }

    fn is_gear(&self) -> bool {
        matches!(self, Character::Symbol('*'))
    }
}

pub struct Area {
    left_top: Point,
    right_bottom: Point,
}
impl Area {
    fn new(left_top: Point, right_bottom: Point) -> Self {
        Self {
            left_top,
            right_bottom,
        }
    }

    /// Walk the area from left to right, top to bottom
    fn walk_lr_tb(&self) -> impl Iterator<Item = Point> {
        let y_range = self.left_top.y..=self.right_bottom.y;
        let x_range = self.left_top.x..=self.right_bottom.x;

        y_range.flat_map(move |y| x_range.clone().map(move |x| Point::new(x, y)))
    }

    /// Creates a new area, expanded by 1 point, keeping it within the bounds of
    /// `origin..=right_bottom_bound`
    fn expand_one(&self, right_bottom_bound: Point) -> Area {
        let min = Point::new(
            self.left_top.x.saturating_sub(1),
            self.left_top.y.saturating_sub(1),
        );
        let max = Point::new(
            (self.right_bottom.x + 1).min(right_bottom_bound.x),
            (self.right_bottom.y + 1).min(right_bottom_bound.y),
        );
        Area::new(min, max)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Point {
    x: usize,
    y: usize,
}
impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn area(&self) -> Area {
        Area::new(*self, *self)
    }
}

impl DayInput for Schematic {
    fn load(input: &'static str) -> Self {
        Schematic {
            rows: Vec::load(input),
        }
    }
}

impl DayInput for Vec<Character> {
    fn load(input: &'static str) -> Self {
        input.chars().map(Character::from_char).collect()
    }
}

impl Character {
    pub fn from_char(ch: char) -> Character {
        match ch {
            '.' => Character::Empty,
            c => match c.to_digit(10) {
                Some(x) => Character::Digit(x),
                None => Character::Symbol(c),
            },
        }
    }
}
