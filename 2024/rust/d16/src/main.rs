use std::collections::{BinaryHeap, HashMap, HashSet};

type Map = Vec<Vec<char>>;
type Pos = (i32, i32);

// Up, right, down, left
const DIRS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
const STEP_C: i32 = 1;
const TURN_C: i32 = 1000;

#[derive(Debug, Eq, PartialEq)]
struct State {
    path: Vec<Pos>,
    dir: usize,
    cost: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let content = include_str!("input.txt");
    let map = map(content);
    let start = start(&map);
    let paths = bfs(&map, &start);

    let min_cost = paths.iter().map(|(_, cost)| cost).min().unwrap();
    let mut all_min: HashSet<Pos> = HashSet::new();
    for (path, cost) in &paths {
        if cost == min_cost {
            all_min.extend(path.iter());
        }
    }
    println!("Min score: {}", min_cost);
    println!("Great seats: {}", all_min.len());
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

fn bfs(map: &Map, start: &Pos) -> Vec<(Vec<Pos>, i32)> {
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let mut paths: Vec<(Vec<Pos>, i32)> = Vec::new();
    let mut visited: HashMap<(Pos, usize), i32> = HashMap::new();
    assert_ne!(map[start.1 as usize][start.0 as usize], '#');
    heap.push(State {
        path: vec![*start],
        dir: 1,
        cost: 0,
    });

    while let Some(State { path, dir, cost }) = heap.pop() {
        let pos = path.last().unwrap();
        if visited.contains_key(&(*pos, dir)) && visited[&(*pos, dir)] < cost {
            continue;
        }
        visited.insert((*pos, dir), cost);
        let (x, y) = *pos;
        if map[y as usize][x as usize] == 'E' {
            paths.push((path, cost));
            continue;
        }
        let (dx, dy) = DIRS[dir];
        let new_pos = (x + dx, y + dy);
        if map[new_pos.1 as usize][new_pos.0 as usize] != '#' {
            let mut path = path.clone();
            path.push(new_pos);
            heap.push(State {
                path,
                dir,
                cost: cost + STEP_C,
            });
        }
        for &new_dir in adj_dir(dir).iter() {
            let mut path = path.clone();
            path.push(*pos);
            heap.push(State {
                path,
                dir: new_dir,
                cost: cost + TURN_C,
            });
        }
    }
    paths
}

fn adj_dir(dir: usize) -> [usize; 2] {
    match dir {
        0 => [1, 3],
        1 => [0, 2],
        2 => [1, 3],
        3 => [0, 2],
        _ => panic!("Invalid direction"),
    }
}
