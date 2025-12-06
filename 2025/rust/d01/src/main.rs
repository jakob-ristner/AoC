use std::cmp::{max, min};

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{i32, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::preceded,
};

const GRID_SIZE: i32 = 100;

#[derive(Debug)]
enum Command {
    Left(i32),
    Right(i32),
}

fn main() {
    let input = include_str!("../input.txt");
    let (_, commands) = parse_commands(input).expect("Failed to parse commands");
    let (states, xx) = all_states(&commands);

    let zero_count = states.iter().filter(|&&pos| pos % GRID_SIZE == 0).count();

    println!("p1: {}", zero_count);
    println!("p2: {:?}", xx);
}

fn parse_commands(input: &str) -> IResult<&str, Vec<Command>> {
    let left_parser = map(preceded(tag("L"), i32), Command::Left);
    let right_parser = map(preceded(tag("R"), i32), Command::Right);
    separated_list1(line_ending, alt((left_parser, right_parser))).parse(input)
}

fn apply_command(position: i32, command: &Command) -> (i32, i32) {
    let diff = match command {
        Command::Left(dist) => -dist,
        Command::Right(dist) => *dist,
    };

    let wraps = zero_passes(position, diff);

    (position + diff, wraps)
}

fn zero_passes(position: i32, diff: i32) -> i32 {
    let start = position;
    let end = position + diff;

    let pos_start = (start % GRID_SIZE + GRID_SIZE) % GRID_SIZE;
    let pos_end = (end % GRID_SIZE + GRID_SIZE) % GRID_SIZE;

    unimplemented!()
}

fn all_states(commands: &[Command]) -> (Vec<i32>, i32) {
    let mut states = Vec::with_capacity(commands.len() + 1);
    let mut position = 50;
    let mut ctr = 0;
    states.push(position);
    for command in commands {
        let (res, xx) = apply_command(position, command);
        ctr += xx;
        position = res;
        states.push(position);
    }
    println!("Number of wraps: {}", ctr);
    (states, ctr)
}
