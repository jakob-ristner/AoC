use std::collections::{HashMap, HashSet, VecDeque};
type Pos = (u32, u32);
type Graph = HashMap<Pos, Vec<(u32, Pos)>>;

fn main() {
    let content = include_str!("input.txt");
    let (graph, starts) = build_graph(content);
    let p1: u32 = starts
        .iter()
        .map(|start| paths_to_peak(&graph, *start, false)).sum();
    let p2: u32 = starts
        .iter()
        .map(|start| paths_to_peak(&graph, *start, true)).sum();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}


fn paths_to_peak(graph: &Graph, start: Pos, distinct: bool) -> u32 {
    let mut stack = VecDeque::new();
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut ctr = 0;
    stack.push_back((0, start));
    while let Some((height, pos)) = stack.pop_front()
    {
        if !distinct && visited.contains(&pos) {
            continue;
        }
        if height == 9 {
            ctr += 1;
            if !distinct {
                visited.insert(pos);
            }
            continue;
        }
        for (adj_height, adj_pos) in &graph[&pos] {
            if *adj_height != height + 1 {
                continue;
            }
            stack.push_back((height + 1, *adj_pos));
        }
    }
    ctr
}

fn build_graph(input: &str) -> (Graph, Vec<Pos>) {
    let mut graph = Graph::new();

    let w = input.lines().next().unwrap().len() as u32;
    let h = input.lines().count() as u32;

    let charmat = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for y in 0..h {
        for x in 0..w {
            let pos = (x, y);
            let adj = adj(&pos, w as usize, h as usize)
                .into_iter()
                       .filter(|&(x, y)| charmat[y as usize][x as usize].is_ascii_digit())
                .map(|(x, y)| {
                    let c = charmat[y as usize][x as usize].to_digit(10).unwrap();
                    (c, (x, y))
                })
                .collect::<Vec<_>>();
            graph.insert(pos, adj);
        }
    }

    let starts = charmat
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, &c)| c == '0')
                .map(move |(x, _)| (x as u32, y as u32))
        })
        .collect::<Vec<_>>();

    (graph, starts)
}

fn adj(pos: &Pos, w: usize, h: usize) -> Vec<Pos> {
    let mut res = Vec::new();
    if pos.0 > 0 {
        res.push((pos.0 - 1, pos.1));
    }
    if pos.0 < w as u32 - 1 {
        res.push((pos.0 + 1, pos.1));
    }
    if pos.1 > 0 {
        res.push((pos.0, pos.1 - 1));
    }
    if pos.1 < h as u32 - 1 {
        res.push((pos.0, pos.1 + 1));
    }
    res
}
