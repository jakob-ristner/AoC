use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{i64, line_ending, multispace0};
use nom::combinator::all_consuming;
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, terminated};
use nom::{IResult, Parser};
use std::collections::HashMap;

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

fn rectangle_inside(
    p1: &Point,
    p2: &Point,
    vertices: &Vec<Point>,
    cache: &mut HashMap<Point, bool>,
) -> bool {
    let min_x = p1.x.min(p2.x);
    let max_x = p1.x.max(p2.x);
    let min_y = p1.y.min(p2.y);
    let max_y = p1.y.max(p2.y);

    for x in min_x..=max_x {
        let p = Point { x, y: min_y };
        if !*cache.entry(p).or_insert_with(|| point_inside(&p, vertices)) {
            return false;
        }
        let p = Point { x, y: max_y };
        if !*cache.entry(p).or_insert_with(|| point_inside(&p, vertices)) {
            return false;
        }
    }
    for y in (min_y + 1)..max_y {
        let p = Point { x: min_x, y };
        if !*cache.entry(p).or_insert_with(|| point_inside(&p, vertices)) {
            return false;
        }
        let p = Point { x: max_x, y };
        if !*cache.entry(p).or_insert_with(|| point_inside(&p, vertices)) {
            return false;
        }
    }
    true
}

fn area(p1: &Point, p2: &Point) -> i64 {
    ((p1.x - p2.x).abs() + 1) * ((p1.y - p2.y).abs() + 1)
}

fn main() {
    let input = include_str!("../test.txt");
    let (_, points) = parse(input).unwrap();

    let mut cache = HashMap::new();

    let max_1 = points
        .iter()
        .combinations(2)
        .map(|pair| (pair[0], pair[1], area(pair[0], pair[1])))
        .max_by_key(|(_, _, area)| *area);
    println!("{:?}", max_1);

    let max_2 = points
        .iter()
        .combinations(2)
        .map(|pair| (pair[0], pair[1], area(pair[0], pair[1])))
        .sorted_by_key(|(_, _, area)| std::cmp::Reverse(*area))
        .find_map(|(p1, p2, area)| {
            if rectangle_inside(p1, p2, &points, &mut cache) {
                Some(area)
            } else {
                None
            }
        });

    println!("{:?}", max_2);
}

fn point_inside(point: &Point, vertices: &Vec<Point>) -> bool {
    let n = vertices.len();
    let mut winding = 0;
    let Point { x: px, y: py } = point;

    for i in 0..n {
        let Point { x: x1, y: y1 } = vertices[i];
        let Point { x: x2, y: y2 } = vertices[(i + 1) % n];

        let cross = (x1 - px) * (y2 - py) - (x2 - px) * (y1 - py);

        if cross == 0 {
            if x1.min(x2) <= *px && *px <= x1.max(x2) && y1.min(y2) <= *py && *py <= y1.max(y2) {
                return true;
            }
        }

        if y1 <= *py {
            if y2 > *py && cross > 0 {
                winding += 1;
            }
        } else {
            if y2 <= *py && cross < 0 {
                winding -= 1;
            }
        }
    }

    winding != 0
}
