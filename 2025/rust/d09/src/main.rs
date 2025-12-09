use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{i64, line_ending, multispace0};
use nom::combinator::all_consuming;
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, terminated};
use nom::{IResult, Parser};
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

fn parse(input: &str) -> IResult<&str, Vec<Point>> {
    all_consuming(terminated(separated_list1(line_ending, point), multispace0)).parse(input)
}

fn point(input: &str) -> IResult<&str, Point> {
    separated_pair(i64, tag(","), i64)
        .map(|(x, y)| Point { x, y })
        .parse(input)
}

fn rectangle_inside(p1: &Point, p2: &Point, vertices: &[Point]) -> bool {
    let min_x = p1.x.min(p2.x);
    let max_x = p1.x.max(p2.x);
    let min_y = p1.y.min(p2.y);
    let max_y = p1.y.max(p2.y);

    let n = vertices.len();
    for i in 0..n {
        let Point { x: x1, y: y1 } = vertices[i];
        let Point { x: x2, y: y2 } = vertices[(i + 1) % n];
        if (x1 <= min_x && x2 <= min_x)
            || (x1 >= max_x && x2 >= max_x)
            || (y1 <= min_y && y2 <= min_y)
            || (y1 >= max_y && y2 >= max_y)
        {
            continue;
        }
        return false;
    }
    true
}

fn area((p1, p2): &(Point, Point)) -> i64 {
    ((p1.x - p2.x).abs() + 1) * ((p1.y - p2.y).abs() + 1)
}

fn main() {
    let input = include_str!("../input.txt");
    let (_, points) = parse(input).unwrap();

    let areas: Vec<_> = points
        .clone()
        .into_iter()
        .tuple_combinations()
        .map(|(p1, p2)| (p1, p2, area(&(p1, p2))))
        .sorted_by_key(|&(_, _, area)| std::cmp::Reverse(area))
        .collect();

    let (_, _, p1) = areas[0];

    let p2 = areas
        .par_iter()
        .find_first(|(p1, p2, _)| rectangle_inside(p1, p2, &points))
        .map(|(_, _, area)| *area)
        .unwrap();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
