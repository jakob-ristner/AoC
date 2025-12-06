use nom::{
    branch::alt,
    character::complete::line_ending,
    combinator::{opt, value},
    multi::fold_many1,
};

use fxhash::FxHashMap;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, space1},
    IResult,
};

use super::*;
pub fn parse(input: &str) -> IResult<&str, (Units, Lookups)> {
    let (input, lookups) = fold_many1(lookup, FxHashMap::default, |mut acc, (id, value)| {
        acc.insert(id, value);
        acc
    })(input)?;
    let (input, _) = line_ending(input)?;
    let (input, units) = fold_many1(gate, FxHashMap::default, |mut acc, (out, in1, op, in2)| {
        acc.insert(out, (in1, op, in2));
        acc
    })(input)?;
    Ok((input, (units, lookups)))
}

fn lookup(input: &str) -> IResult<&str, (String, bool)> {
    let (input, id) = alphanumeric1(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = space1(input)?;
    let (input, value) = alt((value(true, tag("1")), value(false, tag("0"))))(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, (id.into(), value)))
}

fn gate(input: &str) -> IResult<&str, (String, String, Op, String)> {
    let (input, in1) = alphanumeric1(input)?;
    let (input, _) = space1(input)?;
    let (input, op) = alt((
        value(Op::OR, tag("OR")),
        value(Op::AND, tag("AND")),
        value(Op::XOR, tag("XOR")),
    ))(input)?;
    let (input, _) = space1(input)?;
    let (input, in2) = alphanumeric1(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("->")(input)?;
    let (input, _) = space1(input)?;
    let (input, out) = alphanumeric1(input)?;
    let (input, _) = opt(line_ending)(input)?;
    Ok((input, (out.into(), in1.into(), op, in2.into())))
}
