use std::{cell::RefCell, rc::Rc};

use fxhash::FxHashMap;
type Units = FxHashMap<String, (String, Op, String)>;
type Lookups = FxHashMap<String, bool>;

mod parsing;
use crate::parsing::parse;

#[derive(Debug, Clone, Copy)]
enum Op {
    OR,
    AND,
    XOR,
}

// Part 2 solved visually by hand.
fn main() {
    let content = include_str!("input.txt");
    let (_, (units, lookups)) = parse(content).unwrap();
    let mut lookups = lookups;
    let p1 = calc(&mut lookups, &units);
    println!("Part 1: {:?}", p1);
}

fn calc(lookups: &mut Lookups, units: &Units) -> u64 {
    let mut id = 0;
    let mut out = 0;
    let mut id_str = num_to_z(id);
    while let Some(value) = eval(lookups, units, &id_str) {
        if value {
            out += 2_u64.pow(id);
        }
        id += 1;
        id_str = num_to_z(id);
    }
    out
}

fn num_to_z(num: u32) -> String {
    format!("z{:02}", num)
}

fn eval(lookups: &mut Lookups, units: &Units, id: &str) -> Option<bool> {
    if let Some(value) = lookups.get(id) {
        return Some(*value);
    }
    let (in1, op, in2) = units.get(id)?;
    let in1 = eval(lookups, units, &in1)?;
    let in2 = eval(lookups, units, &in2)?;
    let value = match op {
        Op::OR => in1 || in2,
        Op::AND => in1 && in2,
        Op::XOR => in1 != in2,
    };
    lookups.insert(id.into(), value);
    Some(value)
}
