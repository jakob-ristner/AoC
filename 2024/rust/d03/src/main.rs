use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, i32},
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

use nom::combinator::map;
use std::time::Instant;

fn mul(input: &str) -> IResult<&str, (i32, i32)> {
    delimited(tag("mul("), separated_pair(i32, tag(","), i32), tag(")"))(input)
}

fn p1(input: &str) -> IResult<&str, i32> {
    let (input, v) = many1(many_till(anychar, mul).map(|(_, (a, b))| a * b))(input)?;
    Ok((input, v.iter().sum()))
}
#[derive(Debug)]
enum Op {
    Mul(i32, i32),
    Do,
    Dont,
}
fn p2(input: &str) -> IResult<&str, i32> {
    let (input, ins) = many1(
        many_till(
            anychar,
            alt((
                map(mul, |(a, b)| Op::Mul(a, b)),
                map(tag("do()"), |_| Op::Do),
                map(tag("don't()"), |_| Op::Dont),
            )),
        )
        .map(|(_, v)| v),
    )(input)?;
    let (_, acc): (i32, i32) = ins.iter().fold((1, 0), |(proc, acc), op| match op {
        Op::Mul(a, b) => (proc, acc + a * b * proc),
        Op::Do => (1, acc),
        Op::Dont => (0, acc),
    });
    Ok((input, acc))
}

fn main() {
    let content = include_str!("input.txt");
    let time = Instant::now();
    let (_, p1) = p1(content).unwrap();
    let (_, p2) = p2(content).unwrap();
    println!("Time: {}µs", time.elapsed().as_micros());
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
