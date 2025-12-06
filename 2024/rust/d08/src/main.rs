use std::{any, collections::HashMap};

type Pos = (i32, i32);
type Antennas = HashMap<char, Vec<Pos>>;
type AntiNodes = HashMap<Pos, Vec<char>>;

fn main() {
    println!("Hello, world!");
    let content = include_str!("input.txt");
    let (antennas, xbound, ybound) = parse_input(content);
    let res1 = unique_antinodes_p2(&antennas, xbound, ybound);
    let res2 = unique_antinodes_p1(&antennas, xbound, ybound);
    println!("Result 1: {}", res1);
    println!("Result 2: {}", res2);
}

fn unique_antinodes_p1(antennas: &Antennas, xbound: usize, ybound: usize) -> usize {
    let mut anti_nodes = AntiNodes::new();
    for (freq, positions) in antennas {
        let pairs = pairs(positions);
        for pair in pairs {
            let (an1, an2) = get_antinodes_p1(&pair, xbound, ybound);
            if let Some(an1) = an1 {
                anti_nodes.entry(an1).or_default().push(*freq);
            }
            if let Some(an2) = an2 {
                anti_nodes.entry(an2).or_default().push(*freq);
            }
        }
    }
    anti_nodes.keys().count()
}

fn unique_antinodes_p2(antennas: &Antennas, xbound: usize, ybound: usize) -> usize {
    let mut anti_nodes = AntiNodes::new();
    for (freq, positions) in antennas {
        let pairs = pairs(positions);
        for pair in pairs {
            let ans = get_antinodes_p2(&pair, xbound, ybound);
            for an in ans {
                anti_nodes.entry(an).or_default().push(*freq);
            }
        }
    }
    anti_nodes.keys().count()
}

fn pairs(positions: &[Pos]) -> Vec<(Pos, Pos)> {
    let mut pairs = Vec::new();
    for (i, &pos1) in positions.iter().enumerate() {
        for &pos2 in &positions[i + 1..] {
            pairs.push((pos1, pos2));
        }
    }
    pairs
}

fn get_antinodes_p2((pos1, pos2): &(Pos, Pos), xlen: usize, ylen: usize) -> Vec<Pos> {
    let pos1xpos = pos1.0 >= pos2.0;
    let pos1ypos = pos1.1 >= pos2.1;

    let dx = (pos1.0 - pos2.0).abs();
    let dy = (pos1.1 - pos2.1).abs();

    let mut ans = vec![];
    ans.push(*pos1);

    let rdx_step = if pos1xpos { dx } else { -dx };
    let rdy_step = if pos1ypos { dy } else { -dy };
    let mut rdx = rdx_step;
    let mut rdy = rdy_step;
    while pos1.0 + rdx < xlen as i32
        && pos1.1 + rdy < ylen as i32
        && pos1.0 + rdx >= 0
        && pos1.1 + rdy >= 0
    {
        ans.push((pos1.0 + rdx, pos1.1 + rdy));
        rdx += rdx_step;
        rdy += rdy_step;
    }
    let ydx_step = if pos1xpos { -dx } else { dx };
    let ydy_step = if pos1ypos { -dy } else { dy };
    let mut ydx = ydx_step;
    let mut ydy = ydy_step;
    while pos1.0 + ydx < xlen as i32
        && pos1.1 + ydy < ylen as i32
        && pos1.0 + ydx >= 0
        && pos1.1 + ydy >= 0
    {
        ans.push((pos1.0 + ydx, pos1.1 + ydy));
        ydx += ydx_step;
        ydy += ydy_step;
    }
    ans
}

fn get_antinodes_p1(
    (pos1, pos2): &(Pos, Pos),
    xlen: usize,
    ylen: usize,
) -> (Option<Pos>, Option<Pos>) {
    let pos1xpos = pos1.0 >= pos2.0;
    let pos1ypos = pos1.1 >= pos2.1;

    let dx = (pos1.0 - pos2.0).abs();
    let dy = (pos1.1 - pos2.1).abs();

    let (anx1, anx2) = if pos1xpos {
        (pos1.0 + dx, pos2.0 - dx)
    } else {
        (pos1.0 - dx, pos2.0 + dx)
    };
    let (any1, any2) = if pos1ypos {
        (pos1.1 + dy, pos2.1 - dy)
    } else {
        (pos1.1 - dy, pos2.1 + dy)
    };
    let an1 = if anx1 >= xlen as i32 || anx1 < 0 || any1 >= ylen as i32 || any1 < 0 {
        None
    } else {
        Some((anx1, any1))
    };
    let an2 = if anx2 >= xlen as i32 || anx2 < 0 || any2 >= ylen as i32 || any2 < 0 {
        None
    } else {
        Some((anx2, any2))
    };
    (an1, an2)
}

fn parse_input(input: &str) -> (Antennas, usize, usize) {
    let mut antennas = Antennas::new();
    for (y, lin) in input.lines().enumerate() {
        for (x, c) in lin.chars().enumerate() {
            if c != '.' && c != '#' {
                antennas.entry(c).or_default().push((x as i32, y as i32));
            }
        }
    }
    (
        antennas,
        input.lines().next().unwrap().len(),
        input.lines().count(),
    )
}
