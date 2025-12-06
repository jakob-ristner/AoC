use std::{collections::BinaryHeap, u32, vec};

use nom::{
    bytes::complete::tag, character::complete::line_ending, combinator::opt, multi::fold_many0,
    sequence::separated_pair, IResult,
};
extern crate fxhash;
use fxhash::{FxHashMap, FxHashSet};
use nom::character::complete::i32;

type Pos = (i32, i32);
type Bytes = FxHashMap<Pos, usize>;

const TMAX: usize = 1024;
const BOUNDS: i32 = 70;
const GOAL: Pos = (BOUNDS, BOUNDS);

#[derive(Debug, Eq, PartialEq)]
struct State {
    pos: Pos,
    cost: u32,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn bfs(bytes: &Bytes, tmax: usize) -> Option<u32> {
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let mut visited: FxHashSet<Pos> = FxHashSet::default();
    heap.push(State {
        pos: (0, 0),
        cost: 0,
    });

    while let Some(State { pos, cost }) = heap.pop() {
        if pos == GOAL {
            return Some(cost);
        }
        if !visited.insert(pos) {
            continue;
        }

        for adjacent in adj(&pos) {
            if bytes.get(&adjacent).map(|step| *step < tmax).unwrap_or(false) {
                continue;
            }
            heap.push(State {
                pos: adjacent,
                cost: cost + 1,
            });
        }
    }
    None
}

fn main() {
    println!("Hello, world!");
    let content = include_str!("input.txt");
    let (_, hm) = parse(content).unwrap();

    let mut last = 0;
    for n in TMAX.. {
        if let Some(cost) = bfs(&hm, n) {
            last = n;
        } else {
            let byte = hm.iter().filter(|(k, v)| **v == last).last().unwrap();
            dbg!(byte);
            break;
        }
    }
}

fn adj((x, y): &Pos) -> Vec<Pos> {
    let mut adj = vec![];
    if *x > 0 {
        adj.push((x - 1, *y));
    }
    if *x < BOUNDS {
        adj.push((x + 1, *y));
    }
    if *y > 0 {
        adj.push((*x, y - 1));
    }
    if *y < BOUNDS {
        adj.push((*x, y + 1));
    }
    adj
}

fn parse(input: &str) -> IResult<&str, Bytes> {
    let (input, (hm, _)) = fold_many0(
        |input| {
            let (input, (x, y)) = separated_pair(i32, tag(","), i32)(input)?;
            let (input, _) = opt(line_ending)(input)?;
            Ok((input, (x, y)))
        },
        || (Bytes::default(), 0),
        |(mut hm, acc), item| {
            hm.insert(item, acc);
            (hm, acc + 1)
        },
    )(input)?;
    Ok((input, hm))
}