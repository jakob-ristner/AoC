use disjoint::DisjointSet;
use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{i64, line_ending, multispace0},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::terminated,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

fn dist_sq(a: &Point, b: &Point) -> i64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    let dz = a.z - b.z;
    dx * dx + dy * dy + dz * dz
}

fn parse(input: &str) -> IResult<&str, Vec<Point>> {
    all_consuming(terminated(
        separated_list1(
            line_ending,
            (i64, tag(","), i64, tag(","), i64).map(|(x, _, y, _, z)| Point { x, y, z }),
        ),
        multispace0,
    ))
    .parse(input)
}

fn main() {
    let input = include_str!("../input.txt");
    let (_, points) = parse(input).unwrap();

    let mut connections: Vec<_> = points
        .iter()
        .enumerate()
        .combinations(2)
        .map(|pair| {
            let (i, p1) = pair[0];
            let (j, p2) = pair[1];
            let dist = dist_sq(p1, p2);
            (dist, i, j)
        })
        .collect();
    connections.sort_by_key(|(dist, _, _)| *dist);

    let test = connections.len() < 1000;
    let mut disjoint_set = DisjointSet::with_len(points.len());
    for (idx, (_, a, b)) in connections.into_iter().enumerate() {
        disjoint_set.join(a, b);
        if idx == 999 || (test && idx == 9) {
            let p1 = disjoint_set
                .sets()
                .iter()
                .map(|s| s.len())
                .sorted_unstable_by(|a, b| b.cmp(a))
                .take(3)
                .product::<usize>();
            println!("Part 1: {}", p1);
        }
        if disjoint_set.sets().len() == 1 {
            let p2 = points[a].x * points[b].x;
            println!("Part 2: {}", p2);
            break;
        }
    }
}
