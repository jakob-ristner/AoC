use rayon::prelude::*;
use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::value;
use nom::multi::{many1, separated_list1};
use nom::IResult;

type Map = Vec<Vec<Obj>>;
type Pos = (i64, i64);

// Up, Right, Down, Left
const DIRS: [(i64, i64); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
type Dir = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Obj {
    Empty,
    Wall,
    Padding,
}

fn main() {
    let content = include_str!("input.txt");
    let (_, (map, pos)) = map(content).unwrap();
    let (path, _) = sim(&map, pos, 0);
    let p2 = num_loops(&map, pos, &path);
    println!("Part 1: {}", path.keys().count());
    println!("Part 2: {}", p2);
}

fn num_loops(map: &Map, pos: Pos, original_path: &HashMap<Pos, Vec<Dir>>) -> i64 {
    original_path
        .par_iter()
        .filter_map(|((x, y), dirs)| {
            if (*x, *y) == pos {
                return None;
            }
            let mut local_map = map.clone();
            local_map[*y as usize][*x as usize] = Obj::Wall;
            let dir = dirs[0];
            let (dx, dy) = DIRS[dir];
            let start_pos = (*x - dx, *y - dy);
            let (_, r#loop) = sim(&local_map, start_pos, dir);
            Some(r#loop as i64)
        })
        .sum()
}

fn sim(map: &Map, pos: Pos, dir: usize) -> (HashMap<Pos, Vec<Dir>>, bool) {
    let mut visited: HashMap<Pos, Vec<Dir>> = HashMap::new();
    let mut pos = pos;
    let mut dir = dir;
    while !visited.get(&pos).map(|d| d.contains(&dir)).unwrap_or(false) {
        visited.entry(pos).or_default().push(dir);
        let (dx, dy) = DIRS[dir];
        let next_pos = (pos.0 + dx, pos.1 + dy);
        match map[next_pos.1 as usize][next_pos.0 as usize] {
            Obj::Wall => {
                dir = (dir + 1) % 4;
            }
            Obj::Empty => {
                pos = next_pos;
            }
            Obj::Padding => return (visited, false),
        }
    }
    (visited, true)
}

fn pad_map(map: Map) -> Map {
    let mut padded_map = vec![vec![Obj::Padding; map[0].len() + 2]; map.len() + 2];
    for (i, row) in map.iter().enumerate() {
        for (j, obj) in row.iter().enumerate() {
            padded_map[i + 1][j + 1] = *obj;
        }
    }
    padded_map
}

fn get_start(input: &str) -> Pos {
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '^' {
                return ((x + 1) as i64, (y + 1) as i64);
            }
        }
    }
    unreachable!();
}

fn obj(input: &str) -> IResult<&str, Obj> {
    use Obj::*;
    alt((
        value(Empty, tag(".")),
        value(Empty, tag("^")),
        value(Wall, tag("#")),
    ))(input)
}

fn map(input: &str) -> IResult<&str, (Map, Pos)> {
    let start = get_start(input);
    let (input, map) = separated_list1(line_ending, many1(obj))(input)?;
    let map = pad_map(map);
    Ok((input, (map, start)))
}
