use std::collections::{HashMap, HashSet};

type Pos = (i32, i32);
type Garden = HashMap<Pos, char>;
type Perimiters = HashSet<(Pos, Pos, Dir)>;
type Area = HashSet<Pos>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let content = include_str!("input.txt");
    let map = parse(content);
    let (c1, c2) = cost(&map);
    println!("Part 1: {}", c1);
    println!("Part 2: {}", c2);
}

fn cost(map: &Garden) -> (usize, usize) {
    let mut visited = Area::new();
    let mut cost1 = 0;
    let mut cost2 = 0;
    for pos in map.keys() {
        if visited.contains(pos) {
            continue;
        }
        let (area, perimiters) = bfs(map, pos);
        let sides = find_comb_sides(&perimiters);
        visited.extend(area.iter());
        cost1 += area.len() * perimiters.len();
        cost2 += sides.len() * area.len();
    }
    (cost1, cost2)
}

fn parse(input: &str) -> HashMap<Pos, char> {
    let mut map = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert((x as i32, y as i32), c);
        }
    }
    map
}

fn bfs(map: &Garden, start: &Pos) -> (Area, Perimiters) {
    let mut visited = Area::new();
    let mut perimiters = Perimiters::new();
    let plant_type = map[start];
    let mut queue = vec![*start];
    while let Some(pos) = queue.pop() {
        if !visited.insert(pos) {
            continue;
        }
        for next in adjacent(&pos) {
            if map.get(&next) == Some(&plant_type) {
                queue.push(next);
            } else {
                let dir = if next.0 > pos.0 {
                    Dir::Right
                } else if next.0 < pos.0 {
                    Dir::Left
                } else if next.1 > pos.1 {
                    Dir::Down
                } else {
                    Dir::Up
                };
                perimiters.insert((pos, next, dir));
            }
        }
    }
    (visited, perimiters)
}

fn adjacent(pos: &Pos) -> Vec<Pos> {
    let (x, y) = pos;
    vec![
        (x - 1, *y),
        (x + 1, *y),
        (*x, y - 1),
        (*x, y + 1),
    ]
}

fn find_comb_sides(perimiters: &Perimiters) -> Vec<HashSet<(Pos, Pos)>>{
    let mut sides: Vec<HashSet<(Pos, Pos)>> = vec![];
    for (a, b, dir) in perimiters.iter() {
        if sides.iter().any(|s| s.contains(&(*a, *b))) {
            continue;
        }
        let mut queue = vec![(*a, *b, dir)];
        let mut side = HashSet::new();
        while let Some((inside, out, dir)) = queue.pop() {
            if !side.insert((inside, out)) {
                continue;
            }
            let adj = adjacent_perimiter(&(inside, out, *dir));
            for (a, b) in adj {
                if perimiters.contains(&(a, b, *dir)) {
                    queue.push((a, b, dir));
                }
            }
        }
        sides.push(side);
    }
    sides
}

fn adjacent_perimiter(perim: &(Pos, Pos, Dir)) -> Vec<(Pos, Pos)> {
    let (start, end, dir) = perim;
    let (x1, y1) = start;
    let (x2, y2) = end;

    match dir {
        Dir::Up => vec![
            ((x1 - 1, *y1), (x2 - 1, *y2)),
            ((x1 + 1, *y1), (x2 + 1, *y2)),
        ],
        Dir::Down => vec![
            ((x1 - 1, *y1), (x2 - 1, *y2)),
            ((x1 + 1, *y1), (x2 + 1, *y2)),
        ],
        Dir::Left => vec![
            ((*x1, y1 - 1), (*x2, y2 - 1)),
            ((*x1, y1 + 1), (*x2, y2 + 1)),
        ],
        Dir::Right => vec![
            ((*x1, y1 - 1), (*x2, y2 - 1)),
            ((*x1, y1 + 1), (*x2, y2 + 1)),
        ],
    }
}
