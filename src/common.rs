use nom::{combinator::eof, sequence::terminated, IResult};

pub use nom::character::complete::u32;
pub use nom::character::complete::u64;

pub fn parse<'i, P, R>(parser: P, input: &'i str) -> R
where
    P: FnMut(&'i str) -> IResult<&'i str, R>,
{
    let (_, r) = terminated(parser, eof)(input).unwrap();
    r
}
