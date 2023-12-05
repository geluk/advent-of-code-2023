use std::cmp::Ordering;

use nom::{
    bytes::complete::{tag, take_until1},
    character::complete::{digit1, line_ending},
    combinator::map_res,
    multi::many1,
    sequence::{delimited, preceded, terminated},
    IResult,
};

use crate::{input::DayInput, Day};

pub struct Day5;

impl Day for Day5 {
    type Input = Almanac;

    const DAY_NO: usize = 5;

    fn solve_challenge_1(input: &Self::Input) -> u32 {
        input.find_min_location(|s| s.iter().map(|s| SeedCollection::from_start_count(*s, 1)))
            as u32
    }

    fn solve_challenge_2(input: &Self::Input) -> u32 {
        input.find_min_location(|s| {
            s.chunks_exact(2)
                .map(|c| SeedCollection::from_start_count(c[0], c[1]))
        }) as u32
    }
}

pub struct Almanac {
    seeds: Vec<u64>,
    range_maps: Vec<RangeMap>,
}
impl Almanac {
    /// Given a seed builder, find the lowest location where any of the seeds end
    /// up being planted.
    fn find_min_location<'s, F: FnOnce(&'s [u64]) -> I, I: Iterator<Item = SeedCollection>>(
        &'s self,
        seed_builder: F,
    ) -> u64 {
        seed_builder(&self.seeds)
            .flat_map(|seeds| self.cascade(seeds))
            .map(|seeds| seeds.start)
            .min()
            .unwrap()
    }

    /// Cascade a seed collection through multiple range maps.
    /// Each range map may split up the collection into multiple smaller collections.
    fn cascade(&self, collection: SeedCollection) -> Vec<SeedCollection> {
        self.range_maps
            .iter()
            .fold(vec![collection], |collections, map| {
                collections
                    .into_iter()
                    .flat_map(|r| map.transform(r))
                    .collect()
            })
    }
}

#[derive(Clone, Copy)]
pub struct SeedCollection {
    start: u64, // Inclusive
    end: u64,   // Exclusive
}
impl SeedCollection {
    pub fn from_start_count(start: u64, count: u64) -> SeedCollection {
        Self {
            start,
            end: start + count,
        }
    }

    pub fn from_first_last(first: u64, last: u64) -> SeedCollection {
        Self {
            start: first,
            end: last + 1,
        }
    }
}

struct RangeMap {
    ranges: Vec<Range>,
}
impl RangeMap {
    fn new(mut ranges: Vec<Range>) -> Self {
        ranges.sort_by_key(|r| r.source_start);

        Self { ranges }
    }

    fn transform(&self, seed_collection: SeedCollection) -> Vec<SeedCollection> {
        // Several ranges may be able to transform (part of) the seed collection.
        // Find out where they are.
        let first_idx = self
            .find_range_index(seed_collection.start)
            .unwrap_or_else(|b| b);
        let last_idx = self
            .find_range_index(seed_collection.end - 1)
            .map(|i| i + 1)
            .unwrap_or_else(|b| b.min(self.ranges.len()));

        self.ranges[first_idx..last_idx]
            .iter()
            .map(|r| r.translate_collection(seed_collection))
            .collect()
    }

    fn find_range_index(&self, source: u64) -> Result<usize, usize> {
        // This is where the magic happens.
        self.ranges
            .binary_search_by(|candidate| candidate.cmp(source))
    }
}

#[derive(Debug)]
struct Range {
    source_start: u64,
    source_end: u64,
    offset: i64,
}
impl Range {
    fn new(src: u64, dest: u64, count: u64) -> Range {
        let offset = (dest as i64) - (src as i64);

        Self {
            source_start: src,
            source_end: src + count,
            offset,
        }
    }

    pub fn translate(&self, src: u64) -> u64 {
        (src as i64 + self.offset) as u64
    }

    pub fn cmp(&self, value: u64) -> Ordering {
        match value {
            v if v < self.source_start => Ordering::Greater, // self > value
            v if v >= self.source_end => Ordering::Less,     // self < value
            _ => Ordering::Equal,
        }
    }

    fn translate_collection(&self, seed_collection: SeedCollection) -> SeedCollection {
        // This implementation works in practice, but for correctness,
        // it should really return 0-3 seed collections:
        // 0 if there is no overlap whatsoever;
        // 1 fully translated collection if the seed collection is entirely
        //   contained within the range;
        // 2 collections (a translated part and an untranslated part) if the
        //   seed collection is partially outside one bound of the range;
        // 3 collections (one translated part and two untranslated parts) if the
        //   seed collection is partially outside both bounds of the range.
        let start = seed_collection.start.max(self.source_start);
        let end = seed_collection.end.min(self.source_end);

        if start < end {
            let first = start;
            let last = end - 1;
            let first_to_dest = self.translate(first);
            let last_to_dest = self.translate(last);

            SeedCollection::from_first_last(first_to_dest, last_to_dest)
        } else {
            seed_collection
        }
    }
}

impl DayInput for Almanac {
    fn load(input: &'static str) -> Self {
        let (_, almanac) = almanac(input).unwrap();
        almanac
    }
}

fn almanac(i: &'static str) -> IResult<&str, Almanac> {
    let (i, seeds) = seed_list(i)?;
    let (i, maps) = many1(range_map)(i)?;

    Ok((
        i,
        Almanac {
            seeds,
            range_maps: maps,
        },
    ))
}

fn range_map(i: &'static str) -> IResult<&str, RangeMap> {
    let (i, _) = delimited(eol, take_until1("\n"), eol)(i)?;
    let (i, mut ranges) = many1(range)(i)?;

    Ok((i, RangeMap::new(ranges)))
}

fn range(i: &str) -> IResult<&str, Range> {
    let (i, dest) = terminated(number, tag(" "))(i)?;
    let (i, src) = terminated(number, tag(" "))(i)?;
    let (i, count) = terminated(number, eol)(i)?;

    Ok((i, Range::new(src, dest, count)))
}

fn seed_list(i: &str) -> IResult<&str, Vec<u64>> {
    delimited(tag("seeds:"), many1(preceded(tag(" "), number)), eol)(i)
}

fn eol(i: &str) -> IResult<&str, &str> {
    line_ending(i)
}

fn number(i: &str) -> IResult<&str, u64> {
    map_res(digit1, |r: &str| r.parse())(i)
}
