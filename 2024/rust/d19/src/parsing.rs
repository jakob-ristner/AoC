use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, line_ending, space1},
    combinator::eof,
    multi::{many1, many_till},
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

pub fn parse(input: &str) -> IResult<&str, (Vec<Vec<char>>, Vec<Vec<char>>)> {
    separated_pair(towels, line_ending, pats)(input)
}

fn towels(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    many1(towel)(input)
}

fn towel(input: &str) -> IResult<&str, Vec<char>> {
    if input.is_empty() || input.chars().next().unwrap() == '\n' {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::ManyTill,
        )));
    }
    many_till(anychar, alt((preceded(tag(","), space1), line_ending)))
        .map(|(chars, _)| chars)
        .parse(input)
}

fn pats(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    many1(pat)(input)
}

fn pat(input: &str) -> IResult<&str, Vec<char>> {
    if input.is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::ManyTill,
        )));
    }
    many_till(anychar, alt((line_ending, eof)))
        .map(|(chars, _)| chars)
        .parse(input)
}
