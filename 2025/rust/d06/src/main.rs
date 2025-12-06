use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, multispace0, multispace1, space0, space1, u64},
    combinator::value,
    multi::separated_list1,
    sequence::terminated,
};

#[derive(Debug, Clone)]
enum Op {
    Add,
    Mul,
}

fn main() {
    let input = include_str!("../input.txt");
    let rotated = rotate(input);

    let (_, p1) = p1(&input).unwrap();
    let (_, p2) = p2(&rotated).unwrap();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn rotate(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let row_len = lines[0].len();
    let mut rotated: Vec<Vec<char>> = vec![Vec::new(); row_len];
    for line in lines {
        for (i, c) in line.chars().enumerate() {
            rotated[i].push(c);
        }
    }
    let rotated: Vec<String> = rotated
        .into_iter()
        .map(|v| v.into_iter().collect())
        .collect();

    rotated.join("\n")
}

fn p2(input: &str) -> IResult<&str, u64> {
    let (input, nums) =
        terminated(separated_list1(multispace1, parse_rot_block), space0).parse(input)?;
    Ok((input, nums.iter().sum()))
}

fn parse_rot_block(input: &str) -> IResult<&str, u64> {
    let (rem, num1) = u64.parse(input)?;

    let (rem, _) = space0(rem)?;
    let (rem, op) = op(rem)?;
    let (rem, _) = (space0, line_ending, space0).parse(rem)?;
    let (rem, nums) = separated_list1((space0, line_ending, space0), u64).parse(rem)?;

    match op {
        Op::Add => Ok((rem, nums.iter().fold(num1, |acc, x| acc + x))),
        Op::Mul => Ok((rem, nums.iter().fold(num1, |acc, x| acc * x))),
    }
}

fn op(input: &str) -> IResult<&str, Op> {
    alt((value(Op::Mul, tag("*")), value(Op::Add, tag("+")))).parse(input)
}

fn p1(input: &str) -> IResult<&str, u64> {
    let (rem, nums) = separated_list1(multispace1, separated_list1(space1, u64)).parse(input)?;
    let (rem, _) = multispace1.parse(rem)?;
    let (rem, ops) = terminated(separated_list1(multispace1, op), multispace0).parse(rem)?;

    let num = ops.iter().enumerate().fold(0, |acc, (i, op)| match op {
        Op::Add => acc + nums.iter().map(|v| v[i]).sum::<u64>(),
        Op::Mul => acc + nums.iter().map(|v| v[i]).product::<u64>(),
    });
    Ok((rem, num))
}
