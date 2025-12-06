use std::collections::{HashMap, HashSet};

type Map = Vec<Vec<char>>;
type Pos = (i32, i32);

// Up, right, down, left
const DIRS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn main() {
    let content = include_str!("input.txt");
    let map = map(content);
    let start = start(&map);
    let path = path(&map, &start);
    let s_cost = path.len() as i32 - 1;
    let p1 = num_cheats(s_cost, &path, 2, 100);
    let p2 = num_cheats(s_cost, &path, 20, 100);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn num_cheats(s_cost: i32, path: &[Pos], ch_len: i32, min_saved: i32) -> usize {
    let mut acc = 0;
    let p_len = path.len();
    let cost_to_end: HashMap<Pos, i32> = path
        .iter()
        .enumerate()
        .map(|(i, &pos)| (pos, (p_len - i - 1) as i32))
        .collect();
    for (cost_to_here, &pos) in path.iter().enumerate() {
        for (opp, offset) in reachable_in_n_steps(&pos, ch_len) {
            if let Some(cost_from_opp) = cost_to_end.get(&opp) {
                let cost = cost_to_here as i32 + offset + cost_from_opp;
                let saved = s_cost - cost;
                if saved >= min_saved {
                    acc += 1;
                }
            }
        }
    }
    acc
}

fn reachable_in_n_steps(from: &Pos, n: i32) -> Vec<(Pos, i32)> {
    let mut out = Vec::new();
    for dy in -n..=n {
        let left = n - dy.abs();
        for dx in -left..=left {
            let pos = (from.0 + dx, from.1 + dy);
            let d = dx.abs() + dy.abs();
            out.push((pos, d));
        }
    }
    out
}
fn map(input: &str) -> Map {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn start(map: &Map) -> Pos {
    for (y, row) in map.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == 'S' {
                return (x as i32, y as i32);
            }
        }
    }
    panic!("No start position found");
}

fn path(map: &Map, start: &Pos) -> Vec<Pos> {
    let mut pos = *start;
    let mut path = vec![pos];
    let mut visited = HashSet::new();
    while map[pos.1 as usize][pos.0 as usize] != 'E' {
        visited.insert(pos);
        let adj = adj_bounded(&pos, map[0].len(), map.len());
        let next = adj
            .iter()
            .find(|&pos| !visited.contains(pos) && map[pos.1 as usize][pos.0 as usize] != '#')
            .unwrap();
        path.push(*next);
        pos = *next;
    }
    path
}

fn adj_bounded(pos: &Pos, w: usize, h: usize) -> Vec<Pos> {
    let mut adj = Vec::new();
    for &(dx, dy) in DIRS.iter() {
        let new_pos = (pos.0 + dx, pos.1 + dy);
        if new_pos.0 >= 0 && new_pos.0 < w as i32 && new_pos.1 >= 0 && new_pos.1 < h as i32 {
            adj.push(new_pos);
        }
    }
    adj
}
