use nom::character::complete::{i64, line_ending};
use nom::multi::separated_list1;
use nom::sequence::tuple;

use nom::{bytes::complete::tag, IResult};

const BONUS: i64 = 10e12 as i64;

#[derive(Debug)]
struct Game {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    px: i64,
    py: i64,
}

fn main() {
    let content = include_str!("input.txt");
    let (_, games) = games(content).unwrap();
    let bonus_games = bonus_games(&games);
    let p1: i64 = games.iter().map(solve).sum();
    let p2: i64 = bonus_games.iter().map(solve).sum();
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn bonus_games(games: &[Game]) -> Vec<Game> {
    games
        .iter()
        .map(|g| Game {
            ax: g.ax,
            ay: g.ay,
            bx: g.bx,
            by: g.by,
            px: g.px + BONUS,
            py: g.py + BONUS,
        })
        .collect()
}

fn solve(g: &Game) -> i64 {
    let det = g.ax * g.by - g.ay * g.bx;
    let a_c = (g.px * g.by - g.py * g.bx) / det;
    let b_c = (g.ax * g.py - g.ay * g.px) / det;
    if g.ax * a_c + g.bx * b_c == g.px && g.ay * a_c + g.by * b_c == g.py {
        a_c * 3 + b_c
    } else {
        0
    }
}

fn games(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(tuple((line_ending, line_ending)), |input| {
        let (input, _) = tag("Button A: X+")(input)?;
        let (input, ax) = i64(input)?;
        let (input, _) = tag(", Y+")(input)?;
        let (input, ay) = i64(input)?;
        let (input, _) = line_ending(input)?;

        let (input, _) = tag("Button B: X+")(input)?;
        let (input, bx) = i64(input)?;
        let (input, _) = tag(", Y+")(input)?;
        let (input, by) = i64(input)?;
        let (input, _) = line_ending(input)?;

        let (input, _) = tag("Prize: X=")(input)?;
        let (input, px) = i64(input)?;
        let (input, _) = tag(", Y=")(input)?;
        let (input, py) = i64(input)?;

        Ok((
            input,
            Game {
                ax,
                ay,
                bx,
                by,
                px,
                py,
            },
        ))
    })(input)
}
