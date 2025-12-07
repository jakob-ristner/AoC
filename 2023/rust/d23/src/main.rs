use std::collections::VecDeque;

use fxhash::{FxHashMap, FxHashSet};
use itertools::{Itertools, Position};

type Graph = FxHashMap<(i32, i32), Tile>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Tile {
    Wall,
    Forest,
    Slope(Dir),
}

impl Tile {
    fn walkable(&self) -> bool {
        match self {
            Tile::Wall => false,
            Tile::Forest | Tile::Slope(_) => true,
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let (graph, start, goal) = parse(input);

    let p1 = longest_path_dfs(&graph, &start, &goal, true);
    println!("Part 1: {}", p1 - 1);

    let p2 = longest_path_dfs(&graph, &start, &goal, false);
    println!("Part 2: {}", p2);
}
fn adjacent_positions2(pos: &(i32, i32), graph: &Graph) -> Vec<(i32, i32)> {
    let (x, y) = pos;
    match graph.get(pos).unwrap() {
        Tile::Wall | Tile::Forest | Tile::Slope(_) => {
            vec![(*x, y - 1), (*x, y + 1), (x - 1, *y), (x + 1, *y)]
        }
    }
}

fn adjacent_positions(pos: &(i32, i32), graph: &Graph) -> Vec<(i32, i32)> {
    let (x, y) = pos;
    match graph.get(pos).unwrap() {
        Tile::Wall | Tile::Forest => vec![(*x, y - 1), (*x, y + 1), (x - 1, *y), (x + 1, *y)],
        Tile::Slope(dir) => match dir {
            Dir::Up => vec![(*x, y - 1)],
            Dir::Down => vec![(*x, y + 1)],
            Dir::Left => vec![(x - 1, *y)],
            Dir::Right => vec![(x + 1, *y)],
        },
    }
}

fn longest_path_dfs(graph: &Graph, start: &(i32, i32), goal: &(i32, i32), p1: bool) -> i32 {
    let mut max_length = 0;
    let mut visited = FxHashSet::default();
    fn dfs(
        pos: &(i32, i32),
        curr_len: i32,
        visited: &mut FxHashSet<(i32, i32)>,
        max_length: &mut i32,
        goal: &(i32, i32),
        graph: &Graph,
        p1: bool,
    ) {
        if pos == goal {
            if curr_len > *max_length {
                println!("{}", curr_len);
                *max_length = curr_len;
            }
            return;
        }
        let adj = {
            if p1 {
                adjacent_positions(pos, graph)
            } else {
                adjacent_positions2(pos, graph)
            }
        };
        let adj = adj.iter().filter_map(|adj_pos| {
            if let Some(tile) = graph.get(adj_pos)
                && tile.walkable()
            {
                return Some(adj_pos);
            }
            None
        });
        for adj_pos in adj {
            if visited.contains(adj_pos) {
                continue;
            }
            visited.insert(*adj_pos);
            dfs(adj_pos, curr_len + 1, visited, max_length, goal, graph, p1);
            visited.remove(adj_pos);
        }
    }

    dfs(start, 0, &mut visited, &mut max_length, goal, graph, p1);
    max_length
}

fn parse(input: &str) -> (Graph, (i32, i32), (i32, i32)) {
    let mut graph = FxHashMap::default();
    let mut start = None;
    let mut goal = None;
    for (pos, (y, line)) in input.lines().enumerate().with_position() {
        for (x, ch) in line.chars().enumerate() {
            let tile = match ch {
                '#' => Tile::Wall,
                '.' => Tile::Forest,
                '^' => Tile::Slope(Dir::Up),
                'v' => Tile::Slope(Dir::Down),
                '<' => Tile::Slope(Dir::Left),
                '>' => Tile::Slope(Dir::Right),
                _ => panic!("Unknown character in input: {}", ch),
            };
            graph.insert((x as i32, y as i32), tile);
            if ch == '.' && pos == Position::First {
                start = Some((x as i32, y as i32));
            }
            if ch == '.' && pos == Position::Last {
                goal = Some((x as i32, y as i32));
            }
        }
    }
    (graph, start.unwrap(), goal.unwrap())
}
