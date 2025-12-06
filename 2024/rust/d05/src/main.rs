use std::cmp::Ordering;
use std::collections::HashSet;

use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, u32};
use nom::multi::separated_list1;
use nom::sequence::pair;
use nom::Parser;
use nom::{sequence::separated_pair, IResult};

type Relation = HashSet<(u32, u32)>;

fn main() {
    let content = include_str!("input.txt");
    let time = std::time::Instant::now();
    let (_, (re, updates)) = parse(content).unwrap();
    let p1 = p1(&re, &updates);
    let p2 = p2(&re, &updates);
    println!("Time: {}µs", time.elapsed().as_micros());
    println!("{}", p1);
    println!("{}", p2);
}

fn p2(re: &Relation, updates: &[Vec<u32>]) -> u32 {
    updates
        .iter()
        .filter(|update| {
            update
                .windows(2)
                .any(|a| cmp(re, a[0], a[1]) == Ordering::Greater)
        })
        .fold(0, |acc, list| {
            let mut list = list.clone();
            list.sort_by(|a, b| cmp(re, *a, *b));
            acc + list[list.len() / 2]
        })
}

fn cmp(relation: &Relation, a: u32, b: u32) -> Ordering {
    match relation.contains(&(a, b)) {
        true => Ordering::Less,
        false => Ordering::Greater 
    }
}

fn p1(re: &Relation, updates: &[Vec<u32>]) -> u32 {
    updates
        .iter()
        .filter(|update| {
            update
                .windows(2)
                .all(|a| cmp(re, a[0], a[1]) == Ordering::Less)
        })
        .map(|update| update[update.len() / 2])
        .sum()
}

fn parse(input: &str) -> IResult<&str, (Relation, Vec<Vec<u32>>)> {
    separated_pair(
        separated_list1(line_ending, rule).map(|rules| rules.into_iter().collect()),
        pair(line_ending, line_ending),
        separated_list1(line_ending, update),
    )(input)
}

fn update(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag(","), u32)(input)
}

fn rule(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(u32, tag("|"), u32)(input)
}