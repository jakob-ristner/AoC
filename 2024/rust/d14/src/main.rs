use std::collections::{HashMap, HashSet};
use std::thread::sleep;

use nom::character::complete::{i32, line_ending};
use nom::multi::separated_list1;
use nom::{bytes::complete::tag, IResult};

const MAX_X: i32 = 101;
const MAX_Y: i32 = 103;

#[derive(Debug)]
struct Robot {
    p: (i32, i32),
    v: (i32, i32),
}

impl Robot {
    fn sim(&self, t: i32, m_x: i32, m_y: i32) -> (i32, i32) {
        let x = (self.p.0 + t * self.v.0) % m_x;
        let x = if x < 0 { x + m_x } else { x };
        let y = (self.p.1 + t * self.v.1) % m_y;
        let y = if y < 0 { y + m_y } else { y };
        (x, y)
    }
}

fn quadrant_count(positions: &Vec<(i32, i32)>) -> usize {
    let half_w = MAX_X / 2;
    let half_h = MAX_Y / 2;
    let q_0 = positions
        .iter()
        .filter(|(x, y)| x < &half_w && y < &half_h)
        .count();
    let q_1 = positions
        .iter()
        .filter(|(x, y)| x > &half_w && y < &half_h)
        .count();
    let q_2 = positions
        .iter()
        .filter(|(x, y)| x < &half_w && y > &half_h)
        .count();
    let q_3 = positions
        .iter()
        .filter(|(x, y)| x > &half_w && y > &half_h)
        .count();
    q_0 * q_1 * q_2 * q_3
}

fn main() {
    let content = include_str!("input.txt");
    let (_, robots) = parse_input(content).unwrap();
    let positions: Vec<_> = robots.iter().map(|r| r.sim(100, MAX_X, MAX_Y)).collect();
    println!("Quadrant count: {}", quadrant_count(&positions));

    // for i in 0..10000 {
    //     println!("Time: {}", i);
    //     let positions: Vec<_> = robots.iter().map(|r| r.sim(i, MAX_X, MAX_Y)).collect();
    //     let grid = to_grid(&positions);
    //     print_grid(&grid);
    // }
}


type Grid = [[bool; MAX_X as usize]; MAX_Y as usize];

fn to_grid(positions: &Vec<(i32, i32)>) -> [[bool; MAX_X as usize]; MAX_Y as usize] {
    let mut grid = [[false; MAX_X as usize]; MAX_Y as usize];
    for (x, y) in positions {
        grid[*y as usize][*x as usize] = true;
    }
    grid
}

fn print_grid(grid: &Grid) {
    for y in 0..MAX_Y {
        for x in 0..MAX_X {
            print!("{}", if grid[y as usize][x as usize] { '#' } else { '.' });
        }
        println!();
    }
}


fn parse_input(input: &str) -> IResult<&str, Vec<Robot>> {
    separated_list1(line_ending, robot)(input)
}

fn robot(input: &str) -> IResult<&str, Robot> {
    let (input, _) = tag("p=")(input)?;
    let (input, px) = i32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, py) = i32(input)?;
    let (input, _) = tag(" v=")(input)?;
    let (input, vx) = i32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, vy) = i32(input)?;
    Ok((
        input,
        Robot {
            p: (px, py),
            v: (vx, vy),
        },
    ))
}
