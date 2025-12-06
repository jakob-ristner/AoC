use crate::*;
use std::collections::VecDeque;
type Pos = (i32, i32);
const NUMPAD_EMPTY: Pos = (0, 3);
pub const NUMPAD_POSITIONS: [Pos; 11] = [
    (1, 3), // 0
    (0, 2), // 1
    (1, 2), // 2
    (2, 2), // 3
    (0, 1), // 4
    (1, 1), // 5
    (2, 1), // 6
    (0, 0), // 7
    (1, 0), // 8
    (2, 0), // 9
    (2, 3), // A (11)
];

pub enum PadType {
    Num,
}

fn paths_from_to(from: Pos, to: Pos, pt: &PadType) -> Vec<VecDeque<Action>> {
    let mut paths = vec![];
    if from == NUMPAD_EMPTY {
        return paths;
    }

    if from == to {
        paths.push(vec![].into());
    } else {
        let (fx, fy) = from;
        let (tx, ty) = to;
        if fx < tx {
            for mut path in paths_from_to((fx + 1, fy), to, pt) {
                path.push_front(Action::Right);
                paths.push(path);
            }
        } else if fx > tx {
            for mut path in paths_from_to((fx - 1, fy), to, pt) {
                path.push_front(Action::Left);
                paths.push(path);
            }
        }
        if fy < ty {
            for mut path in paths_from_to((fx, fy + 1), to, pt) {
                path.push_front(Action::Down);
                paths.push(path);
            }
        } else if fy > ty {
            for mut path in paths_from_to((fx, fy - 1), to, pt) {
                path.push_front(Action::Up);
                paths.push(path);
            }
        }
    }
    paths
}

pub fn all_paths(seq: &[usize]) -> Vec<Vec<Action>> {
    let mut from = seq[0];
    let mut paths = vec![vec![].into()];
    for &to in seq.iter().skip(1) {
        paths = extend_paths(from, to, paths);
        from = to;
    }
    paths.into_iter().map(|path| path.into()).collect()
}

fn extend_paths(from: usize, to: usize, paths: Vec<VecDeque<Action>>) -> Vec<VecDeque<Action>> {
    let from = NUMPAD_POSITIONS[from];
    let to = NUMPAD_POSITIONS[to];
    let mut paths2 = paths_from_to(from, to, &PadType::Num);
    let paths2 = paths2
        .iter_mut()
        .map(|path| {
            path.push_back(Action::A);
            path
        })
        .collect::<Vec<_>>();

    let paths = {
        let mut out = vec![];
        for path in paths {
            for path2 in paths2.iter() {
                let mut path = path.clone();
                path.extend(path2.iter());
                out.push(path);
            }
        }
        out
    };
    paths
}
