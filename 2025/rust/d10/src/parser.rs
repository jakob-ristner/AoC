use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{char, i64, line_ending, multispace0},
    combinator::{all_consuming, value},
    multi::{many1, separated_list1},
    sequence::{delimited, terminated},
};

#[derive(Debug)]
pub struct Machine {
    pub indicators: Vec<i64>,
    pub buttons: Vec<Vec<i64>>,
    pub joltage: Vec<i64>,
}

fn indicators(input: &str) -> IResult<&str, Vec<i64>> {
    delimited(
        char('['),
        many1(alt((value(1, char('#')), value(0, char('.'))))),
        char(']'),
    )
    .parse(input)
}

fn button(len: usize) -> impl Fn(&str) -> IResult<&str, Vec<i64>> {
    move |input: &str| {
        delimited(
            char('('),
            separated_list1((char(','), multispace0), i64),
            char(')'),
        )
        .map(|btn: Vec<i64>| {
            let mut zeroes = vec![0; len];
            for i in &btn {
                zeroes[*i as usize] = 1;
            }
            zeroes
        })
        .parse(input)
    }
}

fn joltage(input: &str) -> IResult<&str, Vec<i64>> {
    delimited(
        char('{'),
        separated_list1((char(','), multispace0), i64),
        char('}'),
    )
    .parse(input)
}

fn machine(input: &str) -> IResult<&str, Machine> {
    let (input, indicators) = indicators(input)?;
    let (input, _) = multispace0(input)?;
    let (input, buttons) = separated_list1(multispace0, button(indicators.len())).parse(input)?;
    let (input, _) = multispace0(input)?;
    let (input, joltage) = joltage(input)?;

    Ok((
        input,
        Machine {
            indicators,
            buttons,
            joltage,
        },
    ))
}

pub fn parse(input: &str) -> IResult<&str, Vec<Machine>> {
    all_consuming(terminated(
        separated_list1(line_ending, machine),
        multispace0,
    ))
    .parse(input)
}
