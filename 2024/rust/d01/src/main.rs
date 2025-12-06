use nom::branch::alt;
use nom::character::complete::{line_ending, space0, u32};
use nom::multi::fold_many0;
use nom::{combinator::eof, IResult};
use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

fn part1() {
    let content = include_str!("input.txt");
    let (_, (mut v1, mut v2)) = parse(content).unwrap();
    v1.sort();
    v2.sort();

    let mut ctr = 0;
    for i in 0..v1.len() {
        ctr += v1[i].abs_diff(v2[i]);
    }

    println!("{}", ctr);
}

fn part2() {
    let content = include_str!("input.txt");
    let (_, (v1, v2)) = parse(content).unwrap();
    let map = frequency_map(v2);

    let mut ctr = 0;
    for i in v1 {
        let counter = map.get(&i).unwrap_or(&0);
        ctr += counter * i;
    }
    println!("{}", ctr);
}

fn frequency_map(input: Vec<u32>) -> HashMap<u32, u32> {
    let mut map = HashMap::new();
    for i in input {
        let counter = map.entry(i).or_insert(0);
        *counter += 1;
    }
    map
}

fn parse(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    let (input, (v1, v2)) = fold_many0(
        line,
        || (Vec::new(), Vec::new()),
        |(mut acc1, mut acc2), (n1, n2)| {
            acc1.push(n1);
            acc2.push(n2);
            (acc1, acc2)
        },
    )(input)?;

    Ok((input, (v1, v2)))
}

fn line(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, n1) = u32(input)?;
    let (input, _) = space0(input)?;
    let (input, n2) = u32(input)?;
    let (input, _) = alt((line_ending, eof))(input)?;
    Ok((input, (n1, n2)))
}
