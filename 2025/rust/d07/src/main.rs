use std::collections::VecDeque;

use fxhash::{FxHashMap, FxHashSet};

type Pos = (i32, i32);
type Map = FxHashMap<Pos, Cell>;

enum Cell {
    Empty,
    Splitter,
}

fn parse(input: &str) -> (Map, Pos) {
    let mut map = FxHashMap::default();
    let mut start = None;

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let pos = (x as i32, y as i32);
            match ch {
                '.' => {
                    map.insert(pos, Cell::Empty);
                }
                '^' => {
                    map.insert(pos, Cell::Splitter);
                }
                'S' => {
                    map.insert(pos, Cell::Empty);
                    start = Some(pos);
                }
                _ => panic!(),
            }
        }
    }
    (map, start.unwrap())
}

fn all_paths_dp(map: &Map, start: Pos, memo: &mut FxHashMap<Pos, u64>) -> u64 {
    if let Some(&count) = memo.get(&start) {
        return count;
    }
    let mut total_paths = 0;
    let (x, y) = start;
    let next @ (nx, ny) = (x, y + 1);
    match map.get(&next) {
        Some(Cell::Empty) => {
            total_paths += all_paths_dp(map, next, memo);
        }
        Some(Cell::Splitter) => {
            total_paths += all_paths_dp(map, (nx + 1, ny), memo);
            total_paths += all_paths_dp(map, (nx - 1, ny), memo);
        }
        None => {
            total_paths = 1;
        }
    }
    memo.insert(start, total_paths);
    total_paths
}

fn sim(map: &Map, start: Pos) -> u32 {
    let mut visited = FxHashSet::default();
    let mut queue = VecDeque::new();
    let mut beams_reached = 0;
    queue.push_back(start);
    while let Some(pos) = queue.pop_front() {
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);
        let (x, y) = pos;
        let next @ (nx, ny) = (x, y + 1);
        match map.get(&next) {
            Some(Cell::Empty) => {
                queue.push_back(next);
            }
            Some(Cell::Splitter) => {
                beams_reached += 1;
                queue.push_back((nx + 1, ny));
                queue.push_back((nx - 1, ny));
            }
            None => {}
        }
    }
    beams_reached
}

fn main() {
    let input = include_str!("../input.txt");
    let (map, start) = parse(input);
    let p1 = sim(&map, start);
    let p2 = all_paths_dp(&map, start, &mut FxHashMap::default());
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
