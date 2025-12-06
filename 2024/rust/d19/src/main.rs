mod parsing;

use fxhash::FxHashMap;
use parsing::parse;

fn main() {
    let content = include_str!("input.txt");
    let (_, (towels, patterns)) = parse(content).unwrap();

    let mut memo = Memo::default();

    let mapped = patterns
        .iter()
        .map(|pat| num_matches(&pat, &towels, &mut memo))
        .collect::<Vec<_>>();

    let matchable = mapped.iter().fold(0, |acc, num| acc + num.min(&1));
    let all_matches = mapped.iter().sum::<u64>();

    println!("Part 1: {}", matchable);
    println!("Part 2: {}", all_matches);
}

type Memo = FxHashMap<String, u64>;
fn num_matches(rem_pattern: &Vec<char>, towels: &Vec<Vec<char>>, memo: &mut Memo) -> u64 {
    if rem_pattern.is_empty() {
        return 1;
    }
    if let Some(&num) = memo.get(&rem_pattern.iter().collect::<String>()) {
        return num;
    }
    let mut num = 0;
    for tow in towels {
        if rem_pattern.len() < tow.len() {
            continue;
        }
        if rem_pattern.iter().zip(tow.iter()).all(|(r, p)| r == p) {
            num += num_matches(&rem_pattern[tow.len()..].to_vec(), towels, memo);
        }
    }
    memo.insert(rem_pattern.iter().collect(), num);
    num
}
