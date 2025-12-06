use fxhash::FxHashSet;

type Pins = FxHashSet<[usize; 5]>;

fn main() {
    let contents = include_str!("input.txt");
    let (keys, locks) = parse(contents);
    let mut acc = 0;
    for key in &keys {
        let fs = locks.iter().filter(|lock| fits(key, lock)).count();
        acc += fs;
    }
    println!("{}", acc);
}

fn fits(key: &[usize; 5], lock: &[usize; 5]) -> bool {
    for i in 0..5 {
        if key[i] + lock[i] > 5 {
            return false;
        }
    }
    true
}

fn parse(input: &str) -> (Pins, Pins) {
    let mut keys = Pins::default();
    let mut locks = Pins::default();
    for pins in input.split("\n\n") {
        let charmat: Vec<Vec<_>> = pins.lines().map(|line| line.chars().collect()).collect();
        let pins = pin_numbers(&charmat[1..charmat.len() - 1]);
        if charmat[0][0] == '#' {
            locks.insert(pins);
        } else {
            keys.insert(pins);
        }
    }
    (keys, locks)
}

fn pin_numbers(input: &[Vec<char>]) -> [usize; 5] {
    let mut pins = [0; 5];
    for x in 0..input[0].len() {
        for y in 0..input.len() {
            if input[y][x] == '#' {
                pins[x] += 1;
            }
        }
    }
    pins
}
