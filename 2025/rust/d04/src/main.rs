use std::collections::{HashMap, HashSet};

type Map = HashSet<(i32, i32)>;
const DELTAS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn main() {
    let input = include_str!("../input.txt");
    let map = parse(input);
    let p1 = p1(&map);
    let p2 = p2(&map);
    println!("p1: {}\np2: {}", p1, p2);
}

fn parse(input: &str) -> Map {
    let mut map = Map::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '@' {
                let res = map.insert((x as i32, y as i32));
                assert!(res)
            }
        }
    }

    return map;
}

fn p2(map: &Map) -> i32 {
    let mut count = 0;
    let mut map = map.clone();

    loop {
        let mut tbr = vec![];
        for pos in map.iter() {
            let adjacent = adjacent_chars(&map, *pos);
            if adjacent < 4 {
                tbr.push(*pos);
            }
        }
        if tbr.is_empty() {
            break;
        }
        for pos in tbr.iter() {
            map.remove(pos);
        }
        count += tbr.len() as i32;
    }
    return count;
}

fn p1(map: &Map) -> i32 {
    let mut count = 0;
    for pos in map.iter() {
        let adjacent = adjacent_chars(map, *pos);
        if adjacent < 4 {
            count += 1;
        }
    }
    return count;
}

fn adjacent_chars(map: &Map, pos: (i32, i32)) -> i32 {
    let mut result = 0;

    for delta in DELTAS.iter() {
        let neighbor_pos = (pos.0 + delta.0, pos.1 + delta.1);
        if map.contains(&neighbor_pos) {
            result += 1;
        }
    }

    return result;
}
