use std::iter::once;

use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

type Graph = FxHashMap<String, FxHashSet<String>>;

fn main() {
    let content = include_str!("input.txt");
    let graph = parse(content);

    let max_cliques = get_max_cliques(&graph);
    let p1 = part1(&max_cliques);
    let p2 = part2(&max_cliques);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn part1(max_cliques: &Vec<FxHashSet<String>>) -> usize {
    max_cliques
        .iter()
        .filter(|clique| clique.len() >= 3)
        .flat_map(|clique| clique.iter().combinations(3))
        .filter(|subclique| subclique.iter().any(|c| c.starts_with('t')))
        .map(|mut subclique| {
            subclique.sort();
            subclique.into_iter().cloned().collect::<Vec<String>>()
        })
        .collect::<FxHashSet<Vec<String>>>()
        .len()
}

fn part2(max_cliques: &Vec<FxHashSet<String>>) -> String {
    let max_clique = max_cliques
        .iter()
        .max_by_key(|clique| clique.len())
        .unwrap();
    let mut sorted_clique: Vec<_> = max_clique.iter().cloned().collect();
    sorted_clique.sort();
    sorted_clique.join(",")
}

fn parse(input: &str) -> Graph {
    let mut graph = Graph::default();
    for line in input.lines() {
        let nodes = line.split("-").collect::<Vec<&str>>();
        let node1 = nodes[0];
        let node2 = nodes[1];
        graph
            .entry(node1.to_string())
            .or_default()
            .insert(node2.to_string());
        graph
            .entry(node2.to_string())
            .or_default()
            .insert(node1.to_string());
    }
    graph
}

fn get_max_cliques(graph: &Graph) -> Vec<FxHashSet<String>> {
    let vertices: FxHashSet<String> = graph.keys().cloned().collect();
    let mut max_cliques = vec![];
    bron_kerbosch(
        &FxHashSet::default(),
        &vertices,
        &FxHashSet::default(),
        &graph,
        &mut max_cliques,
    );
    max_cliques
}

fn bron_kerbosch(
    r: &FxHashSet<String>,
    p: &FxHashSet<String>,
    x: &FxHashSet<String>,
    graph: &FxHashMap<String, FxHashSet<String>>,
    max_cliques: &mut Vec<FxHashSet<String>>,
) {
    if p.is_empty() && x.is_empty() {
        max_cliques.push(r.clone());
        return;
    }
    let pivot = p
        .union(x)
        .max_by_key(|v| {
            graph
                .get(*v)
                .map_or(0, |neighbors| neighbors.intersection(p).count())
        })
        .cloned();
    let temp = FxHashSet::default();
    let pivot_neighbors = match pivot {
        Some(v) => graph.get(&v).unwrap_or(&temp),
        None => return,
    };
    let mut candidates: Vec<_> = p.difference(pivot_neighbors).cloned().collect();

    let mut p = p.clone();
    let mut x = x.clone();
    while let Some(v) = candidates.pop() {
        let temp = FxHashSet::default();
        let v_neighbors = graph.get(&v).unwrap_or(&temp);
        let new_r: FxHashSet<_> = r.iter().chain(once(&v)).cloned().collect();
        let next_p: FxHashSet<_> = p.intersection(v_neighbors).cloned().collect();
        let next_x: FxHashSet<_> = x.intersection(v_neighbors).cloned().collect();
        bron_kerbosch(&new_r, &next_p, &next_x, graph, max_cliques);
        p.remove(&v);
        x.insert(v);
    }
}
