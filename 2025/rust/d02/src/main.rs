use std::{convert::identity, process::id};

use nom::{IResult, NomRange, Parser, bytes::complete::tag, character::complete::i64};

#[derive(Debug)]
struct Range {
    start: i64,
    end: i64,
}

fn main() {
    let input = include_str!("../input.txt");
    let (_, ranges) = parse(input).expect("Failed to parse input");

    let (p1, p2) = ranges
        .iter()
        .map(range_errors)
        .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));
    println!("Part 1: {}\nPart 2: {}", p1, p2);
}

fn parse(input: &str) -> IResult<&str, Vec<Range>> {
    nom::multi::separated_list1(tag(","), range).parse(input)
}

fn range(input: &str) -> IResult<&str, Range> {
    let (input, start) = i64(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, end) = i64(input)?;
    Ok((input, Range { start, end }))
}

fn range_errors(range: &Range) -> (i64, i64) {
    (range.start..=range.end)
        .into_iter()
        .fold((0, 0), |(accp1, accp2), num| {
            (
                accp1 + is_repeated_sequence(num, true),
                accp2 + is_repeated_sequence(num, false),
            )
        })
}

fn is_repeated_sequence(num: i64, p1: bool) -> i64 {
    let s = num.to_string();
    let len = s.len();
    return (1..(len / 2 + 1))
        .into_iter()
        .filter(|i| len % i == 0 && (!p1 || len / i == 2))
        .map(|i| {
            let segment = &s[0..i];
            let repeated = segment.repeat(len / i);
            (repeated == s) as i64 * num
        })
        .sum();
}
