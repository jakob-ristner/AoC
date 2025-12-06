const W_SIZE: usize = 4;
const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
const MAS: [char; 3] = ['M', 'A', 'S'];
const SAM: [char; 3] = ['S', 'A', 'M'];

fn main() {
    let content = include_str!("input.txt");
    let time = std::time::Instant::now();
    let map = parse(content);
    let p1 = p1(&map);
    let p2 = p2(&map);
    println!("P1: {}\nP2: {}\nTime: {:?}", p1, p2, time.elapsed());
}

#[derive(Debug)]
enum Dir {
    Right,
    Down,
    Left,
    Up,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

fn parse(content: &str) -> Vec<Vec<char>> {
    content.lines().map(|line| line.chars().collect()).collect()
}

fn p1(map: &[Vec<char>]) -> u32 {
    let mut acc = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'X' {
                acc += calc_pos_p1(map, x, y);
            }
        }
    }
    acc
}

fn p2(map: &[Vec<char>]) -> u32 {
    let mut acc = 0;
    let width = map[0].len();
    let height = map.len();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'A' {
                acc += calc_pos_p2(map, x, y, width, height);
            }
        }
    }
    acc
}

fn calc_pos_p2(map: &[Vec<char>], x: usize, y: usize, width: usize, height: usize) -> u32 {
    let possible = x > 0 && x < width - 1 && y > 0 && y < height - 1;

    if !possible {
        return 0;
    }

    let first_diag = [map[y - 1][x - 1], map[y][x], map[y + 1][x + 1]];
    let second_diag = [map[y + 1][x - 1], map[y][x], map[y - 1][x + 1]];

    let first_diag_match = first_diag == MAS || first_diag == SAM;
    let second_diag_match = second_diag == MAS || second_diag == SAM;

    if first_diag_match && second_diag_match {
        return 1;
    }

    0
}

fn calc_pos_p1(map: &[Vec<char>], x: usize, y: usize) -> u32 {
    use Dir::*;
    let mut dirs: Vec<Dir> = vec![];
    let width = map[0].len();
    let height = map.len();

    let x_space_min = x.checked_sub(W_SIZE - 1).is_some();
    let x_space_max = x + W_SIZE <= width;
    let y_space_min = y.checked_sub(W_SIZE - 1).is_some();
    let y_space_max = y + W_SIZE <= height;

    if y_space_min {
        dirs.push(Up);
    }
    if y_space_max {
        dirs.push(Down);
    }
    if x_space_min {
        dirs.push(Left);
    }
    if x_space_max {
        dirs.push(Right);
    }
    if x_space_min && y_space_min {
        dirs.push(UpLeft);
    }
    if x_space_max && y_space_min {
        dirs.push(UpRight);
    }
    if x_space_min && y_space_max {
        dirs.push(DownLeft);
    }
    if x_space_max && y_space_max {
        dirs.push(DownRight);
    }

    let mut acc = 0;
    for dir in dirs {
        let xmas = match dir {
            Right => [map[y][x], map[y][x + 1], map[y][x + 2], map[y][x + 3]] == XMAS,
            Down => [map[y][x], map[y + 1][x], map[y + 2][x], map[y + 3][x]] == XMAS,
            Left => [map[y][x], map[y][x - 1], map[y][x - 2], map[y][x - 3]] == XMAS,
            Up => [map[y][x], map[y - 1][x], map[y - 2][x], map[y - 3][x]] == XMAS,
            UpLeft => {
                [
                    map[y][x],
                    map[y - 1][x - 1],
                    map[y - 2][x - 2],
                    map[y - 3][x - 3],
                ] == XMAS
            }
            UpRight => {
                [
                    map[y][x],
                    map[y - 1][x + 1],
                    map[y - 2][x + 2],
                    map[y - 3][x + 3],
                ] == XMAS
            }
            DownLeft => {
                [
                    map[y][x],
                    map[y + 1][x - 1],
                    map[y + 2][x - 2],
                    map[y + 3][x - 3],
                ] == XMAS
            }
            DownRight => {
                [
                    map[y][x],
                    map[y + 1][x + 1],
                    map[y + 2][x + 2],
                    map[y + 3][x + 3],
                ] == XMAS
            }
        };
        if xmas {
            acc += 1;
        }
    }
    acc
}
