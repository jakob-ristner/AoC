use super::*;

pub fn parse(input: &str) -> (Map, Pos, Vec<Dir>) {
    let (map, dirs) = input.split_once("\n\n").unwrap();
    let start = parse_start(map);
    let map = parse_map(map);
    let dirs = parse_dirs(dirs);
    (map, start, dirs)
}

fn parse_dirs(input: &str) -> Vec<Dir> {
    input.lines().fold(Vec::new(), |mut dirs, line| {
        for c in line.chars() {
            let dir = match c {
                '^' => 0,
                'v' => 1,
                '<' => 2,
                '>' => 3,
                _ => panic!("Unknown direction: {}", c),
            };
            dirs.push(dir);
        }
        dirs
    })
}

fn parse_map(input: &str) -> Map {
    let mut map = Map::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let obj = match c {
                '.' => Obj::Empty,
                '@' => Obj::Empty,
                '#' => Obj::Wall,
                'O' => Obj::Rock,
                _ => panic!("Unknown object: {}", c),
            };
            map.insert((x as i32, y as i32), obj);
        }
    }
    map
}

fn parse_start(input: &str) -> Pos {
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '@' {
                return (x as i32, y as i32);
            }
        }
    }
    unreachable!()
}

