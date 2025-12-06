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
                _ => unreachable!(),
            };
            dirs.push(dir);
        }
        dirs
    })
}

fn parse_map(input: &str) -> Map {
    use Obj::*;
    let mut map = Map::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let (o1, o2) = match c {
                '.' => (Empty, Empty),
                '@' => (Empty, Empty),
                '#' => (Wall, Wall),
                'O' => (RockLeft, RockRight),
                _ => unreachable!(),
            };
            map.insert(((x * 2) as i32, y as i32), o1);
            map.insert(((x * 2 + 1) as i32, y as i32), o2);
        }
    }
    map
}

fn parse_start(input: &str) -> Pos {
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '@' {
                return ((x * 2) as i32, y as i32);
            }
        }
    }
    unreachable!()
}

