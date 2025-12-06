use std::{cmp::max, collections::HashMap};

use nom::{
    IResult, Parser,
    character::{
        complete::{i64, line_ending},
        digit1,
    },
    multi::{many1, separated_list1},
};

type Battery = Vec<i64>;

type State = (Battery, i64, i64);

fn main() {
    let input = include_str!("../input.txt");
    let batteries = parse(input);
    let sum = batteries
        .iter()
        .map(|battery| max_power(battery, 0, 12, &mut HashMap::new()))
        .sum::<i64>();

    println!("Sum of max powers: {}", sum);
}

fn max_power(
    battery: &Battery,
    choice: i64,
    max_choices: i64,
    memo: &mut HashMap<State, i64>,
) -> i64 {
    if let Some(&result) = memo.get(&(battery.clone(), choice, max_choices)) {
        return result;
    }

    if battery.is_empty() || choice >= max_choices {
        return 0;
    }
    let current = battery[0];
    let current_value = current * 10_i64.pow(choice as u32);
    let picked = current_value + max_power(&battery[1..].to_vec(), choice + 1, max_choices, memo);
    let not_picked = max_power(&battery[1..].to_vec(), choice, max_choices, memo);
    let m = max(picked, not_picked);
    memo.insert((battery.clone(), choice, max_choices), m);
    m
}

fn parse(input: &str) -> Vec<Battery> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .rev()
                .collect()
        })
        .collect()
}
