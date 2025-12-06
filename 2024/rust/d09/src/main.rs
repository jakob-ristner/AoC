
fn main() {
    let content = include_str!("input.txt");
    let blocks = parse(content);
    let p1_blocks = defrag_unstable(&blocks);
    let c1 = block_checksum(&p1_blocks);

    let p2_blocks = defrag_stable(&blocks);
    let c2 = block_checksum(&p2_blocks);

    println!("Part 1: {}", c1);
    println!("Part 2: {}", c2);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    File(usize),
    Empty,
}

impl Block {
    fn is_empty(&self) -> bool {
        matches!(self, Block::Empty)
    }
    fn id(&self) -> Option<usize> {
        match self {
            Block::File(id) => Some(*id),
            _ => None,
        }
    }
}

fn block_checksum(blocks: &[Block]) -> usize {
    blocks.iter().enumerate().fold(0, |acc, (i, &b)| {
        acc + i * b.id().unwrap_or(0)
    })
}

fn defrag_unstable (blocks: &[Block]) -> Vec<Block> {
    let mut lo = 0;
    let mut hi = blocks.len() - 1;
    let mut new_blocks = Vec::new();
    while lo <= hi {
        match (blocks[lo], blocks[hi]) {
            (Block::Empty, Block::File(_)) => {
                new_blocks.push(blocks[hi]);
                lo += 1;
                hi -= 1;
            }
            (Block::File(_), Block::Empty) => {
                new_blocks.push(blocks[lo]);
                lo += 1;
                hi -= 1;
            }
            (Block::File(_), Block::File(_)) => {
                new_blocks.push(blocks[lo]);
                lo += 1;
            }
            (Block::Empty, Block::Empty) => {
                hi -= 1;
            }
        }

    }
    new_blocks
}

fn defrag_stable(blocks: &[Block]) -> Vec<Block> {
    let mut new = blocks.to_vec();
    let mut hi = new.len() - 1;

    while hi > 0 {
        let mut file_end = hi;
        while file_end > 0 && new[file_end].is_empty() {
            file_end -= 1;
        }
        file_end += 1;
        let id = new[file_end - 1].id().expect("not a file block");
        let mut file_start = file_end - 1;
        while file_start > 0 && !new[file_start].is_empty() && new[file_start].id().unwrap() == id {
            file_start -= 1;
        }
        file_start += 1;
        let mut lo = 0;
        while lo < file_start {
            if !new[lo].is_empty() {
                lo += 1;
                continue;
            }
            let mut emp = lo;
            while emp < file_start && new[emp].is_empty() {
                emp += 1;
            }
            let emp_len = emp - lo;
            let file_len = file_end - file_start;
            if emp_len < file_len {
                lo = emp;
                continue;
            }
            for i in lo..(lo + file_len) {
                new[i] = new[file_start + i - lo];
            }
            for block in &mut new[file_start..file_end] {
                *block = Block::Empty;
            }
            lo = emp;
        }
        hi = file_start - 1;
    }

    new
}


fn parse(input: &str) -> Vec<Block> {
    let mut blocks = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let f_blocks = chars[i].to_digit(10).unwrap();
        blocks.extend(std::iter::repeat(Block::File(i / 2)).take(f_blocks as usize));
        i += 1;
        if i >= chars.len() {
            break;
        }
        let emp_blocks = chars[i].to_digit(10).unwrap();
        blocks.extend(std::iter::repeat(Block::Empty).take(emp_blocks as usize));
        i += 1;
    }
    blocks
}


