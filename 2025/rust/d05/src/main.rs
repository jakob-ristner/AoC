use std::collections::HashSet;

use nom::bytes::tag;
use nom::character::complete::{line_ending, u64};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::{IResult, Parser};

type Rg = (u64, u64);

fn main() {
    let input = include_str!("../input.txt");
    let (_, (rgs, nums)) = parse(input).expect("Failed to parse input");

    let p1 = p1(&rgs, &nums);
    let p2 = p2(&rgs);

    println!("Part 1: {}\nPart 2: {}", p1, p2);
}

fn p2(rgs: &Vec<Rg>) -> u64 {
    let mut rgs = rgs.clone();
    rgs.sort_by(|&(a, _), &(b, _)| a.cmp(&b));

    let max = rgs.len();
    let mut i = 0;
    let mut next_i = i + 1;
    let mut del = HashSet::new();

    while next_i < max {
        let (s1, e1) = rgs[i];
        let (s2, e2) = rgs[next_i];
        if e1 >= (s2 - 1) {
            rgs[i] = (s1, e2.max(e1));
            del.insert(next_i);
            next_i += 1;
        } else {
            i = next_i;
            next_i += 1;
        }
    }
    rgs.iter()
        .enumerate()
        .filter_map(|(i, (s, e))| {
            if (del.contains(&i)) {
                None
            } else {
                Some(e - s + 1)
            }
        })
        .sum()
}

fn p1(rgs: &Vec<Rg>, nums: &Vec<u64>) -> u64 {
    nums.iter().filter(|n| in_any_range(&rgs, **n)).count() as u64
}

fn parse(input: &str) -> IResult<&str, (Vec<Rg>, Vec<u64>)> {
    separated_pair(
        separated_list1(line_ending, separated_pair(u64, tag("-"), u64)),
        (line_ending, line_ending),
        separated_list1(line_ending, u64),
    )
    .parse(input)
}

fn in_any_range(rgs: &Vec<Rg>, n: u64) -> bool {
    for (start, end) in rgs {
        if n >= *start && n <= *end {
            return true;
        }
    }
    false
}
