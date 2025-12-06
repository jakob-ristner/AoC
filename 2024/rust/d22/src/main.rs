use fxhash::FxHashMap;
use nom::{
    character::complete::{i64, line_ending},
    multi::separated_list1,
    IResult,
};

const MODULUS: i64 = 16777216;

type SeqLooup = FxHashMap<[i64; 4], i64>;

fn main() {
    let content = include_str!("input.txt");
    let (_, numbers) = parse(content).unwrap();
    let p1 = numbers.iter().map(|&n| nth_secret(n, 2000)).sum::<i64>();
    let p2 = best_sequence2(&numbers);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn best_sequence2(numbers: &[i64]) -> i64 {
    let lookups = numbers
        .iter()
        .map(|&n| build_lookup_table(n))
        .collect::<Vec<_>>();

    let mut union = lookups[0].clone();
    for lookup in &lookups[1..] {
        for (key, value) in lookup {
            let entry = union.entry(*key).or_insert(0);
            *entry += value;
        }
    }
    union.values().copied().max().unwrap()
}

fn build_lookup_table(init: i64) -> SeqLooup {
    let mut secret = init;
    let mut secrets = vec![init];
    let mut lookup = SeqLooup::default();
    for _ in 0..2000 {
        secret = next_secret(secret);
        secrets.push(secret);
    }
    let deltas = secrets
        .iter()
        .zip(secrets.iter().skip(1))
        .map(|(a, b)| (b % 10) - (a % 10))
        .collect::<Vec<_>>();
    for i in 4..secrets.len() {
        let key = [deltas[i - 4], deltas[i - 3], deltas[i - 2], deltas[i - 1]];
        lookup.entry(key).or_insert_with(|| secrets[i] % 10);
    }
    lookup
}

fn parse(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(line_ending, i64)(input)
}

fn nth_secret(number: i64, n: usize) -> i64 {
    let mut secret = number;
    for _ in 0..n {
        secret = next_secret(secret);
    }
    secret
}

fn next_secret(secret: i64) -> i64 {
    let secret = ((secret * 64) ^ secret) % MODULUS;
    let secret = ((secret / 32) ^ secret) % MODULUS;
    ((secret * 2048) ^ secret) % MODULUS
}
