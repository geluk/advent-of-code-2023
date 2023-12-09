use std::collections::BTreeMap;

use nom::{
    bytes::complete::tag,
    character::{complete::anychar, streaming::line_ending},
    combinator::{map, map_opt},
    multi::{count, many1},
    sequence::{delimited, pair, terminated, tuple},
    IResult,
};

use crate::{common, input::DayInput, Day};

pub struct Day8;
impl Day for Day8 {
    type Input = (Vec<Instruction>, Map);

    const DAY_NO: usize = 8;

    fn solve_challenge_1((instrs, map): &Self::Input) -> u32 {
        let start = Label(['A', 'A', 'A']);
        let end = Label(['Z', 'Z', 'Z']);
        let instruction_cycle = instrs.iter().copied().cycle();
        Cursor::new(start, map, instruction_cycle).walk_to(|l| l == end)
    }

    fn solve_challenge_2((instrs, map): &Self::Input) -> u32 {
        let answer = map
            .starting_points()
            .map(|start| {
                let instruction_cycle = instrs.iter().copied().cycle();
                Cursor::new(start, map, instruction_cycle).walk_to(|l| l.ends_with('Z')) as u64
            })
            .reduce(least_common_multiple)
            .unwrap();

        println!("The answer is {answer}");
        0
    }
}

fn least_common_multiple(left: u64, right: u64) -> u64 {
    let max = left.max(right);
    let min = left.min(right);

    let mut candidate = max;
    while candidate % min != 0 {
        candidate += max;
    }
    candidate
}

struct Cursor<'m, I> {
    instructions: I,
    map: &'m Map,
    position: Label,
}
impl<'m, I: Iterator<Item = Instruction>> Cursor<'m, I> {
    fn new(start: Label, map: &'m Map, instructions: I) -> Self {
        Self {
            instructions,
            map,
            position: start,
        }
    }

    fn walk_to<P>(&mut self, end_predicate: P) -> u32
    where
        P: Fn(Label) -> bool,
    {
        let mut steps = 0;
        while !end_predicate(self.position) {
            self.step();
            steps += 1;
        }

        steps
    }

    fn step(&mut self) {
        let instruction = self.instructions.next().unwrap();
        let next_position = self.map.junction(&self.position).follow(instruction);
        self.position = next_position;
    }
}

pub struct Map {
    connections: BTreeMap<Label, Junction>,
}
impl Map {
    fn junction(&self, start: &Label) -> &Junction {
        &self.connections[start]
    }

    fn starting_points(&self) -> impl Iterator<Item = Label> + '_ {
        self.connections
            .keys()
            .copied()
            .filter(|k| k.ends_with('A'))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Label([char; 3]);
impl Label {
    fn ends_with(&self, char: char) -> bool {
        self.0[2] == char
    }
}

struct Junction {
    left: Label,
    right: Label,
}
impl Junction {
    fn new(left: Label, right: Label) -> Self {
        Self { left, right }
    }

    fn follow(&self, instruction: Instruction) -> Label {
        match instruction {
            Instruction::Left => self.left,
            Instruction::Right => self.right,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Instruction {
    Left,
    Right,
}

impl DayInput for (Vec<Instruction>, Map) {
    fn load(input: &'static str) -> Self {
        common::parse(instructions_map, input)
    }
}

fn instructions_map(i: &str) -> IResult<&str, (Vec<Instruction>, Map)> {
    pair(terminated(instructions, count(line_ending, 2)), junctions)(i)
}

fn instructions(i: &str) -> IResult<&str, Vec<Instruction>> {
    many1(map_opt(anychar, |c| match c {
        'L' => Some(Instruction::Left),
        'R' => Some(Instruction::Right),
        _ => None,
    }))(i)
}

fn junctions(i: &str) -> IResult<&str, Map> {
    map(
        many1(pair(
            terminated(label, tag(" = ")),
            terminated(junction, line_ending),
        )),
        |t: Vec<(Label, Junction)>| {
            let mut connections = BTreeMap::new();
            connections.extend(t);
            Map { connections }
        },
    )(i)
}

fn junction(i: &str) -> IResult<&str, Junction> {
    map(
        pair(
            delimited(tag("("), label, tag(", ")),
            terminated(label, tag(")")),
        ),
        |(l, r)| Junction::new(l, r),
    )(i)
}

fn label(i: &str) -> IResult<&str, Label> {
    map(tuple((anychar, anychar, anychar)), |(a, b, c)| {
        Label([a, b, c])
    })(i)
}
