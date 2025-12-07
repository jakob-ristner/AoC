use std::collections::HashSet;

use fxhash::FxHashSet;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{i32, line_ending, multispace0},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::terminated,
};

const BOTTOM_Z: i32 = 1;

#[derive(Debug, Clone)]
struct Brick {
    x1: i32,
    y1: i32,
    z1: i32,

    x2: i32,
    y2: i32,
    z2: i32,
}

fn main() {
    let input = include_str!("../input.txt");
    let (_, bricks) = parse_bricks(input).unwrap();
    let (bricks, _) = sim(bricks);

    let (p1, p2) = solve(&bricks);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn sim(bricks: Vec<Brick>) -> (Vec<Brick>, u64) {
    let mut bricks = bricks;
    bricks.sort_by_key(|b| b.z1);

    let mut hs = bricks_to_hs(&bricks);
    let mut fallen = 0;

    let mut moved = true;
    while moved {
        moved = false;
        for brick in bricks.iter_mut() {
            if brick.z1 == BOTTOM_Z {
                continue;
            }
            let positions = get_positions(brick);
            let bots = positions.iter().filter(|(_, _, z)| *z == brick.z1);
            let min_z = bots.map(|pos| ray_down(*pos, &hs)).max().unwrap();
            if min_z == brick.z1 {
                continue;
            }
            moved = true;
            fallen += 1;
            (*brick).z2 = min_z + (brick.z2 - brick.z1);
            (*brick).z1 = min_z;
            positions.iter().for_each(|p| {
                hs.remove(p);
            });
            let new_positions = get_positions(brick);
            assert_eq!(positions.len(), new_positions.len());
            new_positions.iter().for_each(|p| {
                hs.insert(*p);
            });
        }
    }
    return (bricks, fallen);
}

fn solve(bricks: &Vec<Brick>) -> (u64, u64) {
    let mut stable = 0;
    let mut fallen_total = 0;
    for (index, _) in bricks.iter().enumerate() {
        let mut test_bricks = bricks.clone();
        test_bricks.remove(index);
        let (_, _fallen) = sim(test_bricks);
        fallen_total += _fallen;
        if _fallen == 0 {
            stable += 1;
        }
    }
    return (stable, fallen_total);
}

fn ray_down(pos: (i32, i32, i32), hs: &FxHashSet<(i32, i32, i32)>) -> i32 {
    let (x, y, mut z) = pos;
    while z > BOTTOM_Z {
        z -= 1;
        if hs.contains(&(x, y, z)) {
            return z + 1;
        }
    }
    return BOTTOM_Z;
}

fn get_positions(brick: &Brick) -> Vec<(i32, i32, i32)> {
    let mut positions = Vec::new();
    for x in brick.x1..=brick.x2 {
        for y in brick.y1..=brick.y2 {
            for z in brick.z1..=brick.z2 {
                positions.push((x, y, z));
            }
        }
    }
    return positions;
}

fn bricks_to_hs(bricks: &Vec<Brick>) -> FxHashSet<(i32, i32, i32)> {
    let mut set = FxHashSet::default();
    bricks.iter().for_each(|b| {
        get_positions(b).iter().for_each(|p| {
            set.insert(*p);
        });
    });
    return set;
}

fn brick(input: &str) -> IResult<&str, Brick> {
    let (input, (x1, _, y1, _, z1)) = (i32, tag(","), i32, tag(","), i32).parse(input)?;
    let (input, _) = tag("~")(input)?;
    let (input, (x2, _, y2, _, z2)) = (i32, tag(","), i32, tag(","), i32).parse(input)?;

    Ok((
        input,
        Brick {
            x1,
            y1,
            z1,
            x2,
            y2,
            z2,
        },
    ))
}

fn parse_bricks(input: &str) -> IResult<&str, Vec<Brick>> {
    all_consuming(terminated(separated_list1(line_ending, brick), multispace0)).parse(input)
}
