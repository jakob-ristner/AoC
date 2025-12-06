use conversion::all_paths;
use fxhash::FxHashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Action {
    Left,
    Right,
    Up,
    Down,
    A,
}

type Memo = FxHashMap<(Action, Action, u64), u64>;

mod conversion;
use Action::*;
fn main() {
    let contents = include_str!("input.txt");
    let seqs = parse_seqs(contents);
    let p1 = solve(&seqs, 2);
    let p2 = solve(&seqs, 25);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn solve(seqs: &Vec<Vec<usize>>, n: u64) -> u64 {
    let mut acc = 0;
    for seq in seqs {
        let paths = all_paths(seq);
        let values = paths
            .iter()
            .map(|path| calc_seq(path, n))
            .collect::<Vec<_>>();
        let min = values.iter().min().unwrap();
        acc += (min) * (seq[1] as u64 * 100 + seq[2] as u64 * 10 + seq[3] as u64);
    }
    acc
}

fn parse_seqs(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            let chars = line.chars().collect::<Vec<_>>();
            let mut out = Vec::with_capacity(chars.len() + 1);
            out.push(10);
            for c in chars {
                out.push(match c {
                    '0' => 0,
                    '1' => 1,
                    '2' => 2,
                    '3' => 3,
                    '4' => 4,
                    '5' => 5,
                    '6' => 6,
                    '7' => 7,
                    '8' => 8,
                    '9' => 9,
                    'A' => 10,
                    _ => unreachable!(),
                });
            }
            out
        })
        .collect()
}

fn calc_seq(seq: &[Action], n: u64) -> u64 {
    let mut memo = Memo::default();
    let mut acc = 0;
    let mut seqx = vec![A];
    seqx.extend(seq.iter());
    for xs in seqx.windows(2) {
        let (from, to) = (xs[0], xs[1]);
        let actions = actions_needed(from, to, &mut memo, n);
        acc += actions;
    }
    acc
}

fn actions_needed(from: Action, to: Action, memo: &mut Memo, r: u64) -> u64 {
    use Action::*;
    if r == 0 {
        return 1;
    }

    if let Some(&v) = memo.get(&(from, to, r)) {
        return v;
    }

    match (from, to) {
        (Left, Left) => 1,
        (Left, Right) => {
            let a_right = actions_needed(A, Right, memo, r - 1);
            memo.insert((A, Right, r - 1), a_right);
            let right_right = actions_needed(Right, Right, memo, r - 1);
            memo.insert((Right, Right, r - 1), right_right);
            let right_a = actions_needed(Right, A, memo, r - 1);
            memo.insert((Right, A, r - 1), right_a);
            a_right + right_right + right_a
        }
        (Left, Up) => {
            let a_right = actions_needed(A, Right, memo, r - 1);
            memo.insert((A, Right, r - 1), a_right);
            let right_up = actions_needed(Right, Up, memo, r - 1);
            memo.insert((Right, Up, r - 1), right_up);
            let up_a = actions_needed(Up, A, memo, r - 1);
            memo.insert((Up, A, r - 1), up_a);
            a_right + right_up + up_a
        }
        (Left, Down) => {
            let a_right = actions_needed(A, Right, memo, r - 1);
            memo.insert((A, Right, r - 1), a_right);
            let right_a = actions_needed(Right, A, memo, r - 1);
            memo.insert((Right, A, r - 1), right_a);
            a_right + right_a
        }
        (Left, A) => {
            let p1 = {
                let a_right = actions_needed(A, Right, memo, r - 1);
                memo.insert((A, Right, r - 1), a_right);
                let right_right = actions_needed(Right, Right, memo, r - 1);
                memo.insert((Right, Right, r - 1), right_right);
                let right_up = actions_needed(Right, Up, memo, r - 1);
                memo.insert((Right, Up, r - 1), right_up);
                let up_a = actions_needed(Up, A, memo, r - 1);
                memo.insert((Up, A, r - 1), up_a);
                a_right + right_right + right_up + up_a
            };
            let p2 = {
                let a_right = actions_needed(A, Right, memo, r - 1);
                memo.insert((A, Right, r - 1), a_right);
                let right_up = actions_needed(Right, Up, memo, r - 1);
                memo.insert((Right, Up, r - 1), right_up);
                let up_right = actions_needed(Up, Right, memo, r - 1);
                memo.insert((Up, Right, r - 1), up_right);
                let right_a = actions_needed(Right, A, memo, r - 1);
                memo.insert((Right, A, r - 1), right_a);
                a_right + right_up + up_right + right_a
            };
            p1.min(p2)
        }
        (Right, Left) => {
            let a_left = actions_needed(A, Left, memo, r - 1);
            memo.insert((A, Left, r - 1), a_left);
            let left_left = actions_needed(Left, Left, memo, r - 1);
            memo.insert((Left, Left, r - 1), left_left);

            let left_a = actions_needed(Left, A, memo, r - 1);
            memo.insert((Left, A, r - 1), left_a);

            a_left + left_left + left_a
        }
        (Right, Right) => 1,
        (Right, Up) => {
            let p1 = {
                let a_left = actions_needed(A, Left, memo, r - 1);
                memo.insert((A, Left, r - 1), a_left);

                let left_up = actions_needed(Left, Up, memo, r - 1);
                memo.insert((Left, Up, r - 1), left_up);

                let up_a = actions_needed(Up, A, memo, r - 1);
                memo.insert((Up, A, r - 1), up_a);

                a_left + left_up + up_a
            };
            let p2 = {
                let a_up = actions_needed(A, Up, memo, r - 1);
                memo.insert((A, Up, r - 1), a_up);

                let up_left = actions_needed(Up, Left, memo, r - 1);
                memo.insert((Up, Left, r - 1), up_left);

                let left_a = actions_needed(Left, A, memo, r - 1);
                memo.insert((Left, A, r - 1), left_a);

                a_up + up_left + left_a
            };
            p1.min(p2)
        }
        (Right, Down) => {
            let a_left = actions_needed(A, Left, memo, r - 1);
            memo.insert((A, Left, r - 1), a_left);
            let left_a = actions_needed(Left, A, memo, r - 1);
            memo.insert((Left, A, r - 1), left_a);
            a_left + left_a
        }
        (Right, A) => {
            let a_up = actions_needed(A, Up, memo, r - 1);
            memo.insert((A, Up, r - 1), a_up);
            let up_a = actions_needed(Up, A, memo, r - 1);
            memo.insert((Up, A, r - 1), up_a);
            a_up + up_a
        }
        (Up, Left) => {
            let a_down = actions_needed(A, Down, memo, r - 1);
            memo.insert((A, Down, r - 1), a_down);
            let down_left = actions_needed(Down, Left, memo, r - 1);
            memo.insert((Down, Left, r - 1), down_left);
            let left_a = actions_needed(Left, A, memo, r - 1);
            memo.insert((Left, A, r - 1), left_a);

            a_down + down_left + left_a
        }
        (Up, Right) => {
            let p1 = {
                let a_down = actions_needed(A, Down, memo, r - 1);
                memo.insert((A, Down, r - 1), a_down);
                let down_right = actions_needed(Down, Right, memo, r - 1);
                memo.insert((Down, Right, r - 1), down_right);
                let right_a = actions_needed(Right, A, memo, r - 1);
                memo.insert((Right, A, r - 1), right_a);
                a_down + down_right + right_a
            };
            let p2 = {
                let a_right = actions_needed(A, Right, memo, r - 1);
                memo.insert((A, Right, r - 1), a_right);
                let right_down = actions_needed(Right, Down, memo, r - 1);
                memo.insert((Right, Down, r - 1), right_down);
                let down_a = actions_needed(Down, A, memo, r - 1);
                memo.insert((Down, A, r - 1), down_a);
                a_right + right_down + down_a
            };
            p1.min(p2)
        }
        (Up, Up) => 1,
        (Up, Down) => {
            let a_down = actions_needed(A, Down, memo, r - 1);
            memo.insert((A, Down, r - 1), a_down);
            let down_a = actions_needed(Down, A, memo, r - 1);
            memo.insert((Down, A, r - 1), down_a);
            a_down + down_a
        }
        (Up, A) => {
            let a_right = actions_needed(A, Right, memo, r - 1);
            memo.insert((A, Right, r - 1), a_right);
            let right_a = actions_needed(Right, A, memo, r - 1);
            memo.insert((Right, A, r - 1), right_a);
            a_right + right_a
        }
        (Down, Left) => {
            let a_left = actions_needed(A, Left, memo, r - 1);
            memo.insert((A, Left, r - 1), a_left);
            let left_a = actions_needed(Left, A, memo, r - 1);
            memo.insert((Left, A, r - 1), left_a);
            a_left + left_a
        }
        (Down, Right) => {
            let a_right = actions_needed(A, Right, memo, r - 1);
            memo.insert((A, Right, r - 1), a_right);
            let right_a = actions_needed(Right, A, memo, r - 1);
            memo.insert((Right, A, r - 1), right_a);
            a_right + right_a
        }
        (Down, Up) => {
            let a_up = actions_needed(A, Up, memo, r - 1);
            memo.insert((A, Up, r - 1), a_up);
            let up_a = actions_needed(Up, A, memo, r - 1);
            memo.insert((Up, A, r - 1), up_a);
            a_up + up_a
        }
        (Down, Down) => 1,
        (Down, A) => {
            let p1 = {
                let a_right = actions_needed(A, Right, memo, r - 1);
                memo.insert((A, Right, r - 1), a_right);
                let right_up = actions_needed(Right, Up, memo, r - 1);
                memo.insert((Right, Up, r - 1), right_up);
                let up_a = actions_needed(Up, A, memo, r - 1);
                memo.insert((Up, A, r - 1), up_a);
                a_right + right_up + up_a
            };
            let p2 = {
                let a_up = actions_needed(A, Up, memo, r - 1);
                memo.insert((A, Up, r - 1), a_up);
                let up_right = actions_needed(Up, Right, memo, r - 1);
                memo.insert((Up, Right, r - 1), up_right);
                let right_a = actions_needed(Right, A, memo, r - 1);
                memo.insert((Right, A, r - 1), right_a);

                a_up + up_right + right_a
            };
            p1.min(p2)
        }
        (A, Left) => {
            let p1 = {
                let a_down = actions_needed(A, Down, memo, r - 1);
                memo.insert((A, Down, r - 1), a_down);

                let down_left = actions_needed(Down, Left, memo, r - 1);
                memo.insert((Down, Left, r - 1), down_left);

                let left_left = actions_needed(Left, Left, memo, r - 1);
                memo.insert((Left, Left, r - 1), left_left);

                let left_a = actions_needed(Left, A, memo, r - 1);
                memo.insert((Left, A, r - 1), left_a);
                a_down + down_left + left_left + left_a
            };
            let p2 = {
                let a_left = actions_needed(A, Left, memo, r - 1);
                memo.insert((A, Left, r - 1), a_left);
                let left_down = actions_needed(Left, Down, memo, r - 1);
                memo.insert((Left, Down, r - 1), left_down);
                let down_left = actions_needed(Down, Left, memo, r - 1);
                memo.insert((Down, Left, r - 1), down_left);
                let left_a = actions_needed(Left, A, memo, r - 1);
                memo.insert((Left, A, r - 1), left_a);

                a_left + left_down + down_left + left_a
            };
            p1.min(p2)
        }
        (A, Right) => {
            let a_down = actions_needed(A, Down, memo, r - 1);
            memo.insert((A, Down, r - 1), a_down);
            let down_a = actions_needed(Down, A, memo, r - 1);
            memo.insert((Down, A, r - 1), down_a);
            a_down + down_a
        }
        (A, Up) => {
            let a_left = actions_needed(A, Left, memo, r - 1);
            memo.insert((A, Left, r - 1), a_left);
            let left_a = actions_needed(Left, A, memo, r - 1);
            memo.insert((Left, A, r - 1), left_a);
            a_left + left_a
        }
        (A, Down) => {
            let p1 = {
                let a_down = actions_needed(A, Down, memo, r - 1);
                memo.insert((A, Down, r - 1), a_down);
                let down_left = actions_needed(Down, Left, memo, r - 1);
                memo.insert((Down, Left, r - 1), down_left);
                let left_a = actions_needed(Left, A, memo, r - 1);
                a_down + down_left + left_a
            };
            let p2 = {
                let a_left = actions_needed(A, Left, memo, r - 1);
                memo.insert((A, Left, r - 1), a_left);
                let left_down = actions_needed(Left, Down, memo, r - 1);
                memo.insert((Left, Down, r - 1), left_down);
                let down_a = actions_needed(Down, A, memo, r - 1);
                memo.insert((Down, A, r - 1), down_a);
                a_left + left_down + down_a
            };
            p1.min(p2)
        }
        (A, A) => 1,
    }
}
