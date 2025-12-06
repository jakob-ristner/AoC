use nom::character::complete::{space1, u64};
use nom::multi::separated_list1;
use nom::IResult;
use std::collections::HashMap;

fn main() {
    let content = include_str!("input.txt");
    let (_, numbers) = parse(content).unwrap();
    let p1 = blink_times(&numbers, 25);
    let p2 = blink_times(&numbers, 75);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn blink_times(nums: &[u64], times: u64) -> u64 {
    nums.iter()
        .fold((0, HashMap::new()), |acc, num| {
            let (acc, mut map) = acc;
            let result = blink(*num, times, &mut map);
            (acc + result, map)
        })
        .0
}

fn parse(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, u64)(input)
}

fn blink(number: u64, times: u64, blink_map: &mut HashMap<(u64, u64), u64>) -> u64 {
    if times == 0 {
        return 1;
    }
    if let Some(&result) = blink_map.get(&(number, times)) {
        return result;
    }

    if number == 0 {
        let result = blink(1, times - 1, blink_map);
        blink_map.insert((number, times), result);
        return result;
    }

    let log10 = (number as f64).log10().floor() as u64;
    if log10 % 2 == 1 {
        let (big_half, small_half) = split_number(number);
        let res1 = blink(big_half, times - 1, blink_map);
        let res2 = blink(small_half, times - 1, blink_map);
        let result = res1 + res2;
        blink_map.insert((number, times), result);
        return result;
    }

    let result = blink(number * 2024, times - 1, blink_map);
    blink_map.insert((number, times), result);
    result
}

fn split_number(number: u64) -> (u64, u64) {
    let log10 = (number as f64).log10().floor() as u64 + 1;
    let half = 10u64.pow((log10 / 2) as u32);
    let small_half = number % half;
    let big_half = (number - small_half) / half;
    (big_half, small_half)
}
