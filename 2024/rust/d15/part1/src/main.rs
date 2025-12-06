use std::collections::HashMap;

mod parsing;
use parsing::parse;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Obj {
    Empty,
    Wall,
    Rock,
}

// Up, Down, Left, Right
const DIRS: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
type Dir = usize;
type Pos = (i32, i32);
type Map = HashMap<Pos, Obj>;

fn main() {
    let content = include_str!("input.txt");
    let (map, start, dirs) = parse(content);
    let map = walk(map, start, &dirs);
    let p1 = gps_sum(&map);
    println!("Part 1: {}", p1);
}

fn gps_sum(map: &Map) -> i32 {
    map.iter().fold(0, |sum, (&(x, y), obj)| match obj {
        Obj::Rock => sum + x + y * 100,
        _ => sum,
    })
}

fn walk(map: Map, start: Pos, dirs: &[Dir]) -> Map {
    let mut pos = start;
    let mut map = map;
    for &dir in dirs {
        let (dx, dy) = DIRS[dir];
        let new_pos = (pos.0 + dx, pos.1 + dy);
        match map.get(&new_pos) {
            Some(Obj::Empty) => pos = new_pos,
            Some(Obj::Rock) => {
                let (n_map, moved) = try_move(map, new_pos, dir);
                if moved {
                    pos = new_pos;
                }
                map = n_map;
            }
            Some(Obj::Wall) => {}
            _ => panic!("Invalid move"),
        }
    }
    map
}

fn try_move(map: Map, pos: Pos, dir: Dir) -> (Map, bool) {
    let (dx, dy) = DIRS[dir];
    let mut new_pos = (pos.0 + dx, pos.1 + dy);
    let mut map = map;
    while let Some(obj) = map.get(&new_pos) {
        match obj {
            Obj::Empty => {
                map.insert(new_pos, Obj::Rock);
                map.insert(pos, Obj::Empty);
                return (map, true);
            }
            Obj::Rock => {
                new_pos = (new_pos.0 + dx, new_pos.1 + dy);
            }
            Obj::Wall => {
                return (map, false);
            }
        }
    }
    todo!()
}
