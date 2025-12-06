use rayon::prelude::*;
use std::vec;

use nom::bytes::complete::tag;
use nom::{character::complete::space1, multi::separated_list1, IResult};
type Line = (u64, Vec<u64>);
use nom::character::complete::{line_ending, u64};

fn main() {
    let cont = include_str!("input.txt");
    let (_, lines) = parse(cont).unwrap();

    let res1: u64 = lines
        .clone()
        .into_iter()
        .filter(possible_p1)
        .map(|l| l.0)
        .sum();
    println!("Part 1: {}", res1);
    let res: u64 = lines.into_par_iter().filter(possible_p2).map(|l| l.0).sum();
    println!("Part 2: {}", res);
}

fn line(input: &str) -> IResult<&str, Line> {
    let (input, number) = u64(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = space1(input)?;
    let (input, numbers) = separated_list1(space1, u64)(input)?;
    Ok((input, (number, numbers)))
}

fn parse(input: &str) -> IResult<&str, Vec<Line>> {
    separated_list1(line_ending, line)(input)
}


fn possible_p2((goal, nums): &Line) -> bool {
    let n = (nums.len() - 1) as u64;
    for i in 0..3u64.pow(n as u32) {
        let mut res = nums[0];
        for j in 0..n {
            res = match (i / 3u64.pow(j as u32)) % 3 {
                0 => res + nums[(j + 1) as usize],
                1 => res * nums[(j + 1) as usize],
                2 => {
                    let mut s = res.to_string();
                    s.push_str(&nums[(j + 1) as usize].to_string());
                    s.parse().unwrap()
                },
                _ => unreachable!(),
            };
            if res > *goal {
                break;
            }
        }
        if res == *goal {
            return true;
        }
    }
    false
}

fn possible_p1((goal, nums): &Line) -> bool {
    let n = (nums.len() - 1) as u64;
    for i in 0..2u64.pow(n as u32) {
        let mut res = nums[0];
        for j in 0..n {
            res = match (i / 2u64.pow(j as u32)) % 2 {
                0 => res + nums[(j + 1) as usize],
                1 => res * nums[(j + 1) as usize],
                _ => unreachable!(),
            };
            if res > *goal {
                break;
            }
        }
        if res == *goal {
            return true;
        }
    }
    false
}
