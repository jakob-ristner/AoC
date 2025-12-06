use core::panic;
use std::collections::{HashMap, HashSet};

mod parsing;
use parsing::parse;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Obj {
    Empty,
    Wall,
    RockRight,
    RockLeft,
}

// Up, Down, Left, Right
const DIRS: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
type Dir = usize;
type Pos = (i32, i32);
type Map = HashMap<Pos, Obj>;

fn main() {
    let content = include_str!("input.txt");
    let p2 = solve(content);
    println!("Part 2: {}", p2);
}

fn solve(content: &str) -> i32 {
    let (map, start, dirs) = parse(content);
    let map = walk(map, start, dirs);
    gps_sum(&map)
}

fn gps_sum(map: &Map) -> i32 {
    map.iter().fold(0, |sum, (&(x, y), obj)| match obj {
        Obj::RockLeft => sum + x + y * 100,
        _ => sum,
    })
}

fn walk(map: Map, start: Pos, dirs: Vec<Dir>) -> Map {
    let mut pos = start;
    let mut map = map;
    for dir in dirs {
        let (dx, dy) = DIRS[dir];
        let new_pos = (pos.0 + dx, pos.1 + dy);
        match map[&new_pos] {
            Obj::Empty => pos = new_pos,
            Obj::Wall => {}
            Obj::RockLeft => {
                if dy == 0 && can_move_hor(&map, new_pos, dx) {
                    map = move_hor(map, new_pos, dx);
                    pos = new_pos;
                } else if dx == 0 {
                    let corr_pos = (new_pos.0 + 1, new_pos.1);
                    let positions = HashSet::from([new_pos, corr_pos]);
                    if can_move_vert(&map, &positions, dy) {
                        map = move_vert(map, positions, dy);
                        pos = new_pos;
                    }
                }
            }
            Obj::RockRight => {
                if dy == 0 && can_move_hor(&map, new_pos, dx) {
                    map = move_hor(map, new_pos, dx);
                    pos = new_pos;
                } else if dx == 0 {
                    let corr_pos = (new_pos.0 - 1, new_pos.1);
                    let positions = HashSet::from([new_pos, corr_pos]);
                    if can_move_vert(&map, &positions, dy) {
                        map = move_vert(map, positions, dy);
                        pos = new_pos;
                    }
                }
            }
        }
    }
    map
}

fn move_hor(map: Map, pos: Pos, dx: i32) -> Map {
    let new_pos = (pos.0 + dx, pos.1);
    let curr_obj = map[&pos];
    let mut map = map;
    match map[&new_pos] {
        Obj::Empty => {
            map.insert(new_pos, curr_obj);
        }
        Obj::RockLeft => {
            map = move_hor(map, new_pos, dx);
            map.insert(new_pos, curr_obj);
            map.insert(pos, Obj::Empty);
        }
        Obj::RockRight => {
            map = move_hor(map, new_pos, dx);
            map.insert(new_pos, curr_obj);
            map.insert(pos, Obj::Empty);
        }
        Obj::Wall => panic!("Invalid move"),
    }
    map
}

fn can_move_hor(map: &Map, pos: Pos, dx: i32) -> bool {
    let new_pos = (pos.0 + dx, pos.1);
    match map[&new_pos] {
        Obj::Empty => true,
        Obj::Wall => false,
        Obj::RockLeft => can_move_hor(map, new_pos, dx),
        Obj::RockRight => can_move_hor(map, new_pos, dx),
    }
}

fn can_move_vert(map: &Map, positions: &HashSet<Pos>, dy: i32) -> bool {
    if positions.is_empty() {
        return true;
    }
    let mut colliding = HashSet::new();
    for (x, y) in positions {
        let new_pos = (*x, y + dy);
        match map[&new_pos] {
            Obj::Empty => (),
            Obj::Wall => return false,
            Obj::RockLeft => {
                colliding.insert(new_pos);
                let corr_pos = (x + 1, y + dy);
                colliding.insert(corr_pos);
            }
            Obj::RockRight => {
                colliding.insert(new_pos);
                let corr_pos = (x - 1, y + dy);
                colliding.insert(corr_pos);
            }
        }
    }
    can_move_vert(map, &colliding, dy)
}

fn move_vert(map: Map, positions: HashSet<Pos>, dy: i32) -> Map {
    if positions.is_empty() {
        return map;
    }
    let mut map = map;
    let mut colliding: HashSet<Pos> = HashSet::new();
    for (x, y) in &positions {
        let new_pos = (*x, y + dy);
        match map[&new_pos] {
            Obj::Empty => (),
            Obj::Wall => panic!("Invalid move"),
            Obj::RockLeft => {
                colliding.insert(new_pos);
                let corr_pos = (x + 1, y + dy);
                colliding.insert(corr_pos);
            }
            Obj::RockRight => {
                colliding.insert(new_pos);
                let corr_pos = (x - 1, y + dy);
                colliding.insert(corr_pos);
            }
        }
    }
    map = move_vert(map, colliding, dy);
    for (x, y) in positions {
        let new_pos = (x, y + dy);
        let curr_obj = map[&(x, y)];
        map.insert(new_pos, curr_obj);
        map.insert((x, y), Obj::Empty);
    }
    map
}
